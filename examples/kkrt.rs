use starknet::{
    accounts::{Account, ConnectedAccount, ExecutionEncoding, SingleOwnerAccount},
    core::types::FieldElement,
    providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, SequencerGatewayProvider},
    signers::{LocalWallet, SigningKey},
};
use starknet_abigen::macros::abigen_legacy;
use starknet_abigen::parser::CairoType;
use std::sync::Arc;

use url::Url;

mod kakarot;
// Example to use an other generated contract from an other module.
// use kakarot::proxy::KkrtProxy;
abigen_legacy!(Kkrt, "./examples/kakarot/abi/kakarot.json");

#[tokio::main]
async fn main() {
    // RPC
    // let rpc_url = Url::parse("<YOUR_NODE_ADDRESS>").expect("Expecting Starknet RPC URL");
    // let provider =
    //     AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(rpc_url.clone())));

    // GATEWAY
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();

    let contract_address = FieldElement::from_hex_be(
        "0x1f9aa338c41c3925c7317d80f0efd6ce3198ab335f6dafffd45b6bead368092",
    )
    .unwrap();

    // If you only plan to call views functions, you can use the `Reader`, which
    // only requires a provider along with your contract address.
    let kkrt_reader = KkrtReader::new(contract_address, &provider);

    // To call a view, there is no need to initialize an account. You can directly
    // use the name of the method in the ABI to realize the call.
    let token = kkrt_reader
        .get_native_token()
        .await
        .expect("Call to `get_native_token` failed");
    println!("TOKEN: {:?}", token);
    assert_eq!(
        token,
        FieldElement::from_hex_be(
            "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7"
        )
        .unwrap()
    );

    // Let's try an eth_call.
    let origin = FieldElement::from_hex_be("0x0").unwrap();
    let to = FieldElement::from_hex_be("0x0").unwrap();
    let gas_limit = FieldElement::from_hex_be("0x0").unwrap();
    let gas_price = FieldElement::from_hex_be("0x0").unwrap();
    let value = FieldElement::from_hex_be("0x0").unwrap();
    let data = vec![FieldElement::ONE, FieldElement::TWO];

    // Example of serialization, where the length is not prefixed.
    let a = Vec::<FieldElement>::serialize(&data);
    println!("a: {:?}", a);

    let r = kkrt_reader
        .eth_call(
            &origin,
            &to,
            &gas_limit,
            &gas_price,
            &value,
            &FieldElement::from(data.len()),
            &data,
        )
        .await;
    println!("{:?}", r);

    // Use an account (example with KATANA).
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

    let kkrt = Kkrt::new(contract_address, account);

    // Let's try an eth_send_transaction.
    let to = FieldElement::from_hex_be("0x0").unwrap();
    let gas_limit = FieldElement::from_hex_be("0x0").unwrap();
    let gas_price = FieldElement::from_hex_be("0x0").unwrap();
    let value = FieldElement::from_hex_be("0x0").unwrap();
    let data = vec![FieldElement::ONE, FieldElement::TWO];

    let r = kkrt
        .eth_send_transaction(
            &to,
            &gas_limit,
            &gas_price,
            &value,
            &FieldElement::from(data.len()),
            &data,
        )
        .await;
    println!("{:?}", r);
}
