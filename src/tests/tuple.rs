use abigen_macro::abigen;
use cairo_types::CairoType;
use starknet::core::types::FieldElement;
use starknet::macros::felt;

#[test]
fn test_tuple_sizes() {
    abigen!(
        ContractA,
        r#"
[
  {
    "type": "struct",
    "name": "contracts::MyStruct",
    "members": [
      {
        "name": "tup2",
        "type": "(core::felt252, core::integer::u32)"
      },
      {
        "name": "tup3",
        "type": "(core::felt252, core::integer::u32, core::integer::u32)"
      },
      {
        "name": "tup4",
        "type": "(core::felt252, core::integer::u32, core::integer::u32, core::felt252)"
      },
      {
        "name": "tup5",
        "type": "(core::felt252, core::integer::u32, core::integer::u32, core::felt252, core::array::Array::<core::u8>)"
      }
    ]
  }
]
"#
    );

    let ms = MyStruct {
        tup2: (FieldElement::THREE, u32::MAX),
        tup3: (FieldElement::THREE, u32::MAX, 14_u32),
        tup4: (FieldElement::THREE, u32::MAX, 14_u32, felt!("0x99999999")),
        tup5: (
            FieldElement::THREE,
            u32::MAX,
            14_u32,
            felt!("0x99999999"),
            vec![0xfe_u8, 0xff_u8],
        ),
    };

    let felts = MyStruct::serialize(&ms);
    assert_eq!(felts.len(), 16);
    assert_eq!(felts[0], FieldElement::THREE);
    assert_eq!(felts[1], FieldElement::from(u32::MAX));
    assert_eq!(felts[2], FieldElement::THREE);
    assert_eq!(felts[3], FieldElement::from(u32::MAX));
    assert_eq!(felts[4], FieldElement::from(14_u32));
    assert_eq!(felts[5], FieldElement::THREE);
    assert_eq!(felts[6], FieldElement::from(u32::MAX));
    assert_eq!(felts[7], FieldElement::from(14_u32));
    assert_eq!(felts[8], felt!("0x99999999"));
    assert_eq!(felts[9], FieldElement::THREE);
    assert_eq!(felts[10], FieldElement::from(u32::MAX));
    assert_eq!(felts[11], FieldElement::from(14_u32));
    assert_eq!(felts[12], felt!("0x99999999"));
    assert_eq!(felts[13], FieldElement::TWO);
    assert_eq!(felts[14], felt!("0xfe"));
    assert_eq!(felts[15], felt!("0xff"));

    let ms2 = MyStruct::deserialize(&felts, 0).unwrap();
    assert_eq!(ms, ms2);
}
