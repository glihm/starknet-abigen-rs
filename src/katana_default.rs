use anyhow::Result;
use starknet::accounts::{SingleOwnerAccount, ExecutionEncoding};
use starknet::core::types::*;
use starknet::macros::felt;
use starknet::providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};
use std::sync::Arc;
use url::Url;

/// Returns a default provider and account for testing purposes on Katana.
pub async fn get_provider_and_account() -> Result<(
    Arc<AnyProvider>,
    Arc<SingleOwnerAccount<Arc<AnyProvider>, LocalWallet>>,
)> {
    let rpc_url = Url::parse("http://0.0.0.0:5050")?;
    let provider = Arc::new(AnyProvider::JsonRpcHttp(JsonRpcClient::new(
        HttpTransport::new(rpc_url.clone()),
    )));

    let account_address =
        felt!("0x517ececd29116499f4a1b64b094da79ba08dfd54a3edaa316134c41f8160973");

    let signer = wallet_from_private_key(&Some(
        "0x0000001800000000300000180000000000030000000000003006001800006600".to_string(),
    ))
    .unwrap();

    let chain_id = provider.chain_id().await?;
    let account = Arc::new(SingleOwnerAccount::new(
        Arc::clone(&provider),
        signer,
        account_address,
        chain_id,
        // Still in legacy for account deployed on katana.
        ExecutionEncoding::Legacy,
    ));

    Ok((provider, account))
}

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
