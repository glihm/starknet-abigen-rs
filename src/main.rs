mod tests;

use abigen_macro::abigen;
use anyhow::Result;
use cairo_types::ty::CairoType;
use cairo_types::types::starknet::*;

mod a;
use a::ContractA;

use url::Url;
use starknet::accounts::{Account, SingleOwnerAccount};
use starknet::core::types::*;
use starknet::macros::felt;
use starknet::providers::{jsonrpc::HttpTransport, AnyProvider, JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};

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
        felt!("0x032be4f29633d261254b1b1c6e7a6889a55354b665a513ef3928409303905631");

    let chain_id = provider.chain_id().await?;
    let account = SingleOwnerAccount::new(&provider, signer, account_address, chain_id);

    let contract_a = ContractA::new(contract_address, &provider).with_account(&account);

    // let m1 = MyStruct {
    //     val: FieldElement::ONE,
    //     val2: felt!("0xffff"),
    // };

    contract_a
        .func1(&ContractAddress(felt!("0x1234")), &EthAddress(felt!("0x222")), &ClassHash(felt!("0x444")))
        .await
        .expect("Fail invoke func1");

    let r = contract_a
        .func2()
        .await
        .expect("Fail call func2");

    println!("\n\n**\n\n{:?}", r);

    let r2 = contract_a
        .func3()
        .await
        .expect("Fail call func3");

    println!("\n\n**\n\n{:?}", r2);
    // let v_get_a = contract_a.get_val().await.expect("Fail call get val");
    // assert_eq!(v_get_a, FieldElement::TWO);
    // let v_get_b = contract_b.get_val().await.expect("Fail call get val");
    // assert_eq!(v_get_b, v_get_a);


    // let contract_b = ContractB::new(contract_address, &provider);

    // contract_a
    //     .set_val(FieldElement::TWO)
    //     .await
    //     .expect("Fail call set val");
    // let v_get_a = contract_a.get_val().await.expect("Fail call get val");
    // assert_eq!(v_get_a, FieldElement::TWO);
    // let v_get_b = contract_b.get_val().await.expect("Fail call get val");
    // assert_eq!(v_get_b, v_get_a);

    // contract_a
    //     .hello_world(FieldElement::THREE)
    //     .await
    //     .expect("Fail call hello world");

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
