use abigen_macro::abigen;
use cairo_types::CairoType;
use starknet::core::types::FieldElement;

#[test]
fn test_option_basic() {
    abigen!(ContractA, "[]");

    let o = Some(FieldElement::THREE);
    let felts = Option::<FieldElement>::serialize(&o);
    assert_eq!(felts[0], FieldElement::ZERO);
    assert_eq!(felts[1], FieldElement::THREE);

    let o2 = Option::<FieldElement>::deserialize(&felts, 0).unwrap();
    assert_eq!(o, o2);

    let o: Option<u32> = None;
    let felts = Option::<u32>::serialize(&o);
    assert_eq!(felts[0], FieldElement::ONE);

    let o2 = Option::<u32>::deserialize(&felts, 0).unwrap();
    assert_eq!(o, o2);
}

#[test]
fn test_option_should_not_be_recreated() {
    abigen!(
        ContractA,
        r#"
[
  {
    "type": "enum",
    "name": "core::option::Option::<core::felt252>",
    "variants": [
      {
        "name": "Some",
        "type": "core::felt252"
      },
      {
        "name": "None",
        "type": "()"
      }
    ]
  },
  {
    "type": "enum",
    "name": "core::option::Option::<core::integer::u32>",
    "variants": [
      {
        "name": "Some",
        "type": "core::integer::u32"
      },
      {
        "name": "None",
        "type": "()"
      }
    ]
  }
]
"#
    );

    let _o = Option::<FieldElement>::Some(FieldElement::ONE);
    // This will clash if Option is actually implemented:
    // 61 | |     );
    //   | |     ^
    //   | |     |
    //   | |_____first implementation here
    //   |       conflicting implementation for `tests::option::test_option_should_not_be_recreated::Option
}
