use abigen_macro::abigen;
use cairo_types::CairoType;
use cairo_types::types::starknet::{ContractAddress, ClassHash, EthAddress};
use starknet::core::types::FieldElement;

#[test]
fn test_starknet_contract_address() {
    abigen!(ContractA, "[]");

    let ca = ContractAddress(FieldElement::ONE);
    let felts = ContractAddress::serialize(&ca);
    assert_eq!(felts[0], FieldElement::ONE);

    let ca2 = ContractAddress::deserialize(&felts, 0).unwrap();
    assert_eq!(ca, ca2);
}

#[test]
fn test_starknet_class_hash() {
    abigen!(ContractA, "[]");

    let ch = ClassHash(FieldElement::ONE);
    let felts = ClassHash::serialize(&ch);
    assert_eq!(felts[0], FieldElement::ONE);

    let ch2 = ClassHash::deserialize(&felts, 0).unwrap();
    assert_eq!(ch, ch2);
}

#[test]
fn test_starknet_eth_address() {
    abigen!(ContractA, "[]");

    let ea = EthAddress(FieldElement::ONE);
    let felts = EthAddress::serialize(&ea);
    assert_eq!(felts[0], FieldElement::ONE);

    let ea2 = EthAddress::deserialize(&felts, 0).unwrap();
    assert_eq!(ea, ea2);
}

#[test]
fn test_starknet_types() {
    abigen!(ContractA, r#"
[
  {
    "type": "struct",
    "name": "contracts::c1::SnTypes",
    "members": [
      {
        "name": "addr",
        "type": "core::starknet::contract_address::ContractAddress"
      },
      {
        "name": "class",
        "type": "core::starknet::class_hash::ClassHash"
      },
      {
        "name": "eth",
        "type": "core::starknet::eth_address::EthAddress"
      }
    ]
  }
]
"#);

    let snt = SnTypes {
        addr: ContractAddress(FieldElement::ONE),
        class: ClassHash(FieldElement::TWO),
        eth: EthAddress(FieldElement::THREE),
    };

    let felts = SnTypes::serialize(&snt);
    assert_eq!(felts[0], FieldElement::ONE);
    assert_eq!(felts[1], FieldElement::TWO);
    assert_eq!(felts[2], FieldElement::THREE);

    let snt2 = SnTypes::deserialize(&felts, 0).unwrap();
    assert_eq!(snt, snt2);
}
