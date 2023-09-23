//! Main with demo for abigen macro usage.
//!
//! If you want to modify the contracts and change the ABI,
//! you can then rebuild the contracts and generate the ABIs by doing:
//!
//! 1. `scarb build` inside the contracts folder at the root of the package.
//! 2. `cargo run -p snabi from-sierra contracts_<contract_name>.sierra.json --expandable src/abis/<contract_name>_abi.rs --name <ContractName>`
//!
//! To test the contract calls, run the following:
//!
//! 1. `katana` to spin up katana on default port 5050.
//! 2. `scarb build` to regenerate the up to date contract code if not already done.
//! 3. Go into contracts folder and run: `make setup_<contract_name>` to deploy the contract.
//! 4. Then you can run the main with `cargo run`.
mod tests;

use anyhow::Result;

mod katana_default;

mod autogen_abis;
use autogen_abis::basic_abi::{u256, BasicContract};
use autogen_abis::gen_abi::{GenContract, MyStruct};
use autogen_abis::event_abi::{EventContract, MyEventA, MyEventB, MyEventC, Event as AnyEvent};

use starknet::core::types::*;
use starknet::providers::Provider;
use starknet::macros::felt;
use cairo_types::{CairoType};
use cairo_types::types::starknet::ContractAddress;

use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let (mut provider, mut account) = katana_default::get_provider_and_account().await?;

    let basic = BasicContract::new(
        felt!("0x04383de1eb63b223170e1de699ff5074fbc1f6096e14604615b65d3d1cc28c7d"),
        Arc::clone(&provider),
    )
        .with_account(Arc::clone(&account));

    let v1 = FieldElement::ONE;
    let v2 = u256 {
        low: 2_u128,
        high: 0_u128,
    };

    basic.set_storage(&v1, &v2).await?;

    let (v1_r, v2_r) = basic.read_storage_tuple().await.unwrap();
    assert_eq!(v1_r, v1);
    assert_eq!(v2_r, v2);

    let gen = GenContract::new(
        felt!("0x0505ca46219e39ede6f186e3056535d82e4eb44bbb49b77531930eeacd1c89e3"),
        Arc::clone(&provider),
    )
        .with_account(Arc::clone(&account));

    let ms = MyStruct {
        f1: FieldElement::ONE,
        f2: FieldElement::TWO,
        f3: FieldElement::THREE,
    };

    gen.func1(&ms).await?;

    // Need to find why there is a nonce error if two invokes are made with the same
    // account.
    // gen.func1(&ms).await?;

    let (f1, f2) = gen.read().await.unwrap();
    assert_eq!(f1, FieldElement::ONE);
    assert_eq!(f2, FieldElement::TWO);

    let event_contract = EventContract::new(
        felt!("0x07912723fd7a18c2b8643db307750b82556b5fc93cfb51bc70c250ed8b9a932b"),
        Arc::clone(&provider),
    )
        .with_account(Arc::clone(&account));

    // Add small delays to avoid nonce error submitting txs too fast.
    event_contract.emit_a(&FieldElement::ONE, &vec![felt!("0xff"), felt!("0xf1")]).await?;
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    event_contract.emit_b(&felt!("0x1234")).await?;
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    event_contract.emit_c(
        &felt!("0x11"),
        &felt!("0x22"),
        &felt!("0x33"),
        &ContractAddress(felt!("0xaa"))
    ).await?;

    let events = katana_default::fetch_all_events(Arc::clone(&provider)).await?;

    for e in events {
        let my_event: AnyEvent = match e.try_into() {
            Ok(ev) => ev,
            Err(s) => {
                // An event from other contracts, ignore.
                continue;
            }
        };

        match my_event {
            AnyEvent::MyEventA(a) => {
                assert_eq!(a.header, FieldElement::ONE);
                assert_eq!(a.value, vec![felt!("0xff"), felt!("0xf1")]);
            }
            AnyEvent::MyEventB(b) => {
                assert_eq!(b.value, felt!("0x1234"));
            }
            AnyEvent::MyEventC(c) => {
                assert_eq!(c.v1, felt!("0x11"));
                assert_eq!(c.v2, felt!("0x22"));
                assert_eq!(c.v3, felt!("0x33"));
                assert_eq!(c.v4, ContractAddress(felt!("0xaa")));
            }
        };
    }

    Ok(())
}
