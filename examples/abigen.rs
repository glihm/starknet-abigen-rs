use starknet::{
    accounts::{Account, ExecutionEncoding, SingleOwnerAccount},
    core::types::FieldElement,
    providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient},
    signers::{LocalWallet, SigningKey},
};
use starknet_abigen::macros::abigen;
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
        "0x005cf74aa753132d4b0fe55f8e16ea23474349d0f1f76aa24051b88e1ea75123",
    )
    .unwrap();

    // If you only plan to call views functions, you can use the `Reader`, which
    // only requires a provider along with your contract address.
    let contract = MyContractReader::new(contract_address, &provider);

    // To call a view, there is no need to initialize an account. You can directly
    // use the name of the method in the ABI to realize the call.
    let a = contract.get_a().await.expect("Call to `get_a` failed");

    println!("a = {:?}", a);

    let b = contract.get_b().await.expect("Call to `get_b` failed");

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

    let account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        FieldElement::from_hex_be("0x4b4154414e41").unwrap(), // KATANA
        ExecutionEncoding::Legacy,
    );

    let contract = MyContract::new(contract_address, &account);

    let r = contract
        .set_a(&(a + FieldElement::ONE))
        .await
        .expect("Call to `set_a` failed");

    // `MyContract` also contains a reader field that you can use if you need both
    // to call external and views with the same instance.

    loop {
        match contract
            .reader
            .get_tx_status(r.transaction_hash)
            .await
            .as_ref()
        {
            "ok" => break,
            "pending" => tokio::time::sleep(tokio::time::Duration::from_secs(1)).await,
            e => {
                println!("Transaction error: {e}");
                break;
            }
        }
    }

    let a = contract
        .reader
        .get_a()
        .await
        .expect("Call to `get_a` failed");

    println!("a = {:?}", a);

    // Now let's say we want to do multicall, and in one transaction we want to set a and b.
    let call_set_a = contract.set_a_getcall(&FieldElement::from_hex_be("0xee").unwrap());
    let call_set_b = contract.set_b_getcall(&u256 { low: 0xff, high: 0 });

    let r = contract
        .account
        .execute(vec![call_set_a, call_set_b])
        .send()
        .await
        .expect("Multicall failed");

    loop {
        match contract
            .reader
            .get_tx_status(r.transaction_hash)
            .await
            .as_ref()
        {
            "ok" => break,
            "pending" => tokio::time::sleep(tokio::time::Duration::from_secs(1)).await,
            e => {
                println!("Transaction error: {e}");
                break;
            }
        }
    }

    let a = contract
        .reader
        .get_a()
        .await
        .expect("Call to `get_a` failed");

    println!("a = {:?}", a);

    let b = contract
        .reader
        .get_b()
        .await
        .expect("Call to `get_b` failed");

    println!("b = {:?}", b);
}
