use starknet::core::types::{BlockId, BlockTag};
use starknet::{core::types::FieldElement, providers::SequencerGatewayProvider};
use starknet_abigen::macros::abigen_legacy;
use starknet_abigen::parser::cairo_types::CairoArrayLegacy;

// Legacy for Cairo 0 support (events are not supported for now).
abigen_legacy!(Kkrt, "./contracts/abi/kakarot.json");

#[tokio::main]
async fn main() {
    // GATEWAY (will be deprecated!).
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();

    let contract_address = FieldElement::from_hex_be(
        "0x1f9aa338c41c3925c7317d80f0efd6ce3198ab335f6dafffd45b6bead368092",
    )
    .unwrap();

    // If you only plan to call views functions, you can use the `Reader`, which
    // only requires a provider along with your contract address.
    let kkrt_reader = KkrtReader::new(contract_address, &provider);

    // Example using the `Latest` block instead of the default value which
    // is `Pending`.
    let token = kkrt_reader
        .get_native_token()
        .block_id(BlockId::Tag(BlockTag::Latest))
        .call()
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
    let data = CairoArrayLegacy(vec![FieldElement::ONE, FieldElement::TWO]);

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
        .call()
        .await;
    println!("{:?}", r);
}
