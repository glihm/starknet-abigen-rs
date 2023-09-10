mod tests;

use abigen_macro::abigen;
use anyhow::Result;
use cairo_types::ty::CairoType;

use starknet::accounts::{Account, SingleOwnerAccount};

use starknet::core::types::*;
use starknet::macros::felt;
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

    let account_address =
        felt!("0x517ececd29116499f4a1b64b094da79ba08dfd54a3edaa316134c41f8160973");

    let signer = wallet_from_private_key(&Some(
        "0x0000001800000000300000180000000000030000000000003006001800006600".to_string(),
    ))
    .unwrap();

    // If you modify the contract, even with a salt, it will be deployed at
    // a different address.
    let contract_address =
        felt!("0x0546a164c8d10fd38652b6426ef7be159965deb9a0cbf3e8a899f8a42fd86761");

    let chain_id = provider.chain_id().await?;
    let account = SingleOwnerAccount::new(&provider, signer, account_address, chain_id);

    let contract_a = ContractA::new(contract_address, &provider).with_account(&account);
    let contract_b = ContractB::new(contract_address, &provider);

    contract_a
        .set_val(FieldElement::TWO)
        .await
        .expect("Fail call set val");
    let v_get_a = contract_a.get_val().await.expect("Fail call get val");
    assert_eq!(v_get_a, FieldElement::TWO);
    let v_get_b = contract_b.get_val().await.expect("Fail call get val");
    assert_eq!(v_get_b, v_get_a);

    contract_a
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
