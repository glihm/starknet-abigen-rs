use starknet::{
    accounts::{ExecutionEncoding, SingleOwnerAccount},
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
abigen!(MyContract, "./examples/abi/simple_read_write_val.abi.json");

#[tokio::main]
async fn main() {
    let rpc_url = Url::parse("http://0.0.0.0:5050").expect("Expecting Starknet RPC URL");
    let provider =
        AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(rpc_url.clone())));

    let contract_address = FieldElement::from_hex_be(
        "0x02fce3db5fa32227b8bef21c76eaf20f8edb29b1662f7fd8920adaa27d75b7b5",
    )
    .unwrap();

    // If you only plan to call views functions, you can use the `Reader`, which
    // only requires a provider along with your contract address.
    let contract = MyContractReader::new(contract_address, &provider);

    // To call a view, there is no need to initialize an account. You can directly
    // use the name of the method in the ABI to realize the call.
    let val = contract
        .read_val()
        .await
        .expect("Call to `read_val` failed");

    println!("Value retrieved: {:?}", val);

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

    contract
        .write_val(&(val + FieldElement::ONE))
        .await
        .expect("Call to `write_val` failed");

    // Some delay to the tx to be processed. Watch the receipt in production
    // code if you need so.
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // `MyContract` also contains a reader field that you can use if you need both
    // to call external and views with the same instance.
    let val = contract
        .reader
        .read_val()
        .await
        .expect("Call to `read_val` failed");

    println!("Value retrieved: {:?}", val);
}
