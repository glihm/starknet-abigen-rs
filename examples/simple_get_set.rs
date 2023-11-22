use starknet::{
    accounts::{Account, ConnectedAccount, ExecutionEncoding, SingleOwnerAccount},
    core::types::{BlockId, BlockTag, FieldElement},
    providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient},
    signers::{LocalWallet, SigningKey},
};
use starknet_abigen::macros::abigen;
use starknet_abigen::parser::call::TransactionStatus;
use std::sync::Arc;
use url::Url;

// Generate the bindings for the contract and also includes
// all the structs and enums present in the ABI with the exact
// same name.
// It's usually a good idea to place the macro call into a separate module
// to avoid name clashes.
abigen!(MyContract, "./contracts/abi/simple_get_set.json");

#[tokio::main]
async fn main() {
    let rpc_url = Url::parse("http://0.0.0.0:5050").expect("Expecting Starknet RPC URL");
    let provider =
        AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(rpc_url.clone())));

    let contract_address = FieldElement::from_hex_be(
        "0x02a293bd31c51a9d3b9ca52bceb77d920523a1cfdde513a95e2a5e792f6e85da",
    )
    .unwrap();

    // If you only plan to call views functions, you can use the `Reader`, which
    // only requires a provider along with your contract address.
    let contract = MyContractReader::new(contract_address, &provider);

    // To call a view, there is no need to initialize an account. You can directly
    // use the name of the method in the ABI to realize the call.
    let a = contract.get_a().call().await.expect("Call to `get_a` failed");
    println!("a {:?}", a);

    // If you need to explicitely set the block id of the call, you can do as
    // following. The default value is "Pending".
    let b = contract.get_b().block_id(BlockId::Tag(BlockTag::Latest)).call().await.expect("Call to `get_b` failed");
    println!("b = {:?}", b);

    // For the inputs / outputs of the ABI functions, all the types are
    // defined where the abigen macro is expanded. Consider using the macro abigen
    // in a separate module to avoid clashes.

    // If you want to do some invoke for external functions, you must use an account.
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be("0x1800000000300000180000000000030000000000003006001800006600")
            .unwrap(),
    ));
    let address = FieldElement::from_hex_be(
        "0x517ececd29116499f4a1b64b094da79ba08dfd54a3edaa316134c41f8160973",
    )
    .unwrap();

    let account = Arc::new(SingleOwnerAccount::new(
        provider,
        signer,
        address,
        FieldElement::from_hex_be("0x4b4154414e41").unwrap(), // KATANA
        ExecutionEncoding::Legacy,
    ));

    let contract = MyContract::new(contract_address, account);

    // The transaction is actually sent when `send()` is called.
    // You can before that configure the fees, or even only run an estimation of the
    // fees without actually sending the transaction.
    let tx_res = contract
        .set_a(&(a + FieldElement::ONE))
        .max_fee(1000000000000000_u128.into())
        .send()
        .await
        .expect("Call to `set_a` failed");

    // Create a new reader from contract account.
    let reader = contract.reader();

    loop {
        // A simple tx watcher that polls the receipt of the transaction hash.
        match reader.get_tx_status(tx_res.transaction_hash).await {
            TransactionStatus::Succeeded => break,
            TransactionStatus::Pending => tokio::time::sleep(tokio::time::Duration::from_secs(1)).await,
            TransactionStatus::Reverted(e) => {
                println!("Transaction reverted: {e}");
                break;
            }
            TransactionStatus::Error(e) => {
                println!("Transaction error: {e}");
                break;
            }
        }
    }

    let a = reader.get_a().call().await.expect("Call to `get_a` failed");
    println!("a = {:?}", a);

    // Now let's say we want to do multicall, and in one transaction we want to set a and b.
    // You can call the same function name with `_getcall` prefix to get the
    // call only, ready to be added in a multicall array.
    let set_a_call = contract.set_a_getcall(&FieldElement::from_hex_be("0xee").unwrap());
    let set_b_call = contract.set_b_getcall(&u256 { low: 0xff, high: 0 });

    // Then, we use the account exposed by the contract to execute the multicall.
    let tx_res = contract
        .account
        .execute(vec![set_a_call, set_b_call])
        .send()
        .await
        .expect("Multicall failed");

    loop {
        match reader.get_tx_status(tx_res.transaction_hash).await {
            TransactionStatus::Succeeded => break,
            TransactionStatus::Pending => tokio::time::sleep(tokio::time::Duration::from_secs(1)).await,
            TransactionStatus::Reverted(e) => {
                println!("Transaction reverted: {e}");
                break;
            }
            TransactionStatus::Error(e) => {
                println!("Transaction error: {e}");
                break;
            }
        }
    }

    let a = reader.get_a().call().await.expect("Call to `get_a` failed");
    println!("a = {:?}", a);

    let b = reader.get_b().call().await.expect("Call to `get_b` failed");
    println!("b = {:?}", b);

    // Remember, ConnectedAccount is implemented for Arc<ConnectedAccount>.
    let arc_contract = Arc::new(contract);

    let handle = tokio::spawn(async move {
        other_func(arc_contract.clone()).await;
    });

    handle.await.unwrap();
}

async fn other_func<A: ConnectedAccount + Sync + 'static>(contract: Arc<MyContract<A>>) {
    // As `Arc<MyContract<A>>` is also implementing `ConnectedAccount`,
    // passing a contract you also have the reader that you can retrieve anytime
    // by calling `contract.reader()`.

    let set_b = contract.set_b(
        &u256 {
            low: 0xfe,
            high: 0,
        }
    );

    // Example of estimation of fees.
    let estimated_fee = set_b.estimate_fee().await.expect("Fail to estimate").overall_fee;

    // Use the estimated fees as a base.
    let tx_res = set_b.max_fee((estimated_fee * 2).into()).send().await.expect("invoke failed");
    println!("{:?}", tx_res);

    let reader = contract.reader();

    loop {
        match reader.get_tx_status(tx_res.transaction_hash).await {
            TransactionStatus::Succeeded => break,
            TransactionStatus::Pending => tokio::time::sleep(tokio::time::Duration::from_secs(1)).await,
            TransactionStatus::Reverted(e) => {
                println!("Transaction reverted: {e}");
                break;
            }
            TransactionStatus::Error(e) => {
                println!("Transaction error: {e}");
                break;
            }
        }
    }

    let b = reader.get_b().call().await.expect("Call to `get_b` failed");
    println!("b = {:?}", b);
}
