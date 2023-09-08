mod tests;

use abigen_macro::abigen;
use anyhow::Result;
use cairo_types::ty::CairoType;

use starknet::accounts::Account;

use starknet::core::types::*;
use starknet::providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};

use url::Url;

abigen!(ContractA, "./test.abi.json");

abigen!(
    ContractB,
    r#"
[
  {
    "type": "function",
    "name": "get_val",
    "inputs": [],
    "outputs": [
      {
        "type": "core::felt252"
      }
    ],
    "state_mutability": "view"
  }
]
"#
);


#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = Url::parse("http://0.0.0.0:5050")?;
    let provider =
        AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(rpc_url.clone())));
    let provider2 =
        AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(rpc_url.clone())));
    let provider3 =
        AnyProvider::JsonRpcHttp(JsonRpcClient::new(HttpTransport::new(rpc_url.clone())));

    let _chain_id = provider.chain_id().await?;

    let account_address = FieldElement::from_hex_be(
        "0x517ececd29116499f4a1b64b094da79ba08dfd54a3edaa316134c41f8160973",
    )
    .unwrap();
    let signer = wallet_from_private_key(&Some(
        "0x0000001800000000300000180000000000030000000000003006001800006600".to_string(),
    ))
    .unwrap();

    // If you modify the contract, even with a salt, it will be deployed at
    // a different address.
    let contract_address = FieldElement::from_hex_be(
        "0x0546a164c8d10fd38652b6426ef7be159965deb9a0cbf3e8a899f8a42fd86761",
    )
    .unwrap();

    let contract_caller_a = ContractA::new_caller(contract_address, provider).await?;

    // TODO: if we can handle the SingleOwnerAccount lifetime, it will be easier to
    // only pass the account. And not account_address + signer.
    let contract_invoker =
        ContractA::new_invoker(contract_address, provider2, account_address, signer).await?;

    let contract_caller_b = ContractB::new_caller(contract_address, provider3).await?;

    contract_invoker
        .set_val(FieldElement::TWO)
        .await
        .expect("Fail call set val");
    let v_get_a = contract_caller_a
        .get_val()
        .await
        .expect("Fail call get val");
    assert_eq!(v_get_a, FieldElement::TWO);
    let v_get_b = contract_caller_b
        .get_val()
        .await
        .expect("Fail call get val");
    assert_eq!(v_get_b, v_get_a);

    contract_invoker
        .hello_world(FieldElement::THREE)
        .await
        .expect("Fail call hello world");

    Ok(())
}

// TODO: Check the class between cairo option and std option.
// we need to pre-declare option in cairo-types and use it with the path cairo_types::Option.
/// Returns a local wallet from a private key, if provided.
fn wallet_from_private_key(
    private_key: &std::option::Option<String>,
) -> std::option::Option<LocalWallet> {
    if let Some(pk) = private_key {
        let private_key = match FieldElement::from_hex_be(pk) {
            Ok(p) => p,
            Err(e) => {
                println!("Error importing private key: {:?}", e);
                return None;
            }
        };
        let key = SigningKey::from_secret_scalar(private_key);
        Some(LocalWallet::from_signing_key(key))
    } else {
        None
    }
}
