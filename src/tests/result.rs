use abigen_macro::abigen;
use cairo_types::CairoType;
use starknet::core::types::FieldElement;

#[test]
fn test_result_basic() {
    abigen!(ContractA, "[]");

    let r = Ok(FieldElement::THREE);
    let felts = Result::<FieldElement, FieldElement>::serialize(&r);
    assert_eq!(felts[0], FieldElement::ZERO);
    assert_eq!(felts[1], FieldElement::THREE);

    let r2 = Result::<FieldElement, FieldElement>::deserialize(&felts, 0).unwrap();
    assert_eq!(r, r2);

    let r = Err(u32::MAX);
    let felts = Result::<FieldElement, u32>::serialize(&r);
    assert_eq!(felts[0], FieldElement::ONE);
    assert_eq!(felts[1], FieldElement::from(u32::MAX));

    let r2 = Result::<FieldElement, u32>::deserialize(&felts, 0).unwrap();
    assert_eq!(r, r2);
}

#[test]
fn test_result_unit() {
    abigen!(ContractA, "[]");

    let r = Ok(());
    let felts = Result::<(), FieldElement>::serialize(&r);
    assert_eq!(felts[0], FieldElement::ZERO);

    let r2 = Result::<(), FieldElement>::deserialize(&felts, 0).unwrap();
    assert_eq!(r, r2);

    let r = Err(FieldElement::TWO);
    let felts = Result::<(), FieldElement>::serialize(&r);
    assert_eq!(felts[0], FieldElement::ONE);
    assert_eq!(felts[1], FieldElement::TWO);

    let r2 = Result::<(), FieldElement>::deserialize(&felts, 0).unwrap();
    assert_eq!(r, r2);
}

#[test]
fn test_result_should_not_be_recreated() {
    abigen!(
        ContractA,
        r#"
[
  {
    "type": "enum",
    "name": "core::result::Result::<(), core::felt252>",
    "variants": [
      {
        "name": "Ok",
        "type": "()"
      },
      {
        "name": "Err",
        "type": "core::felt252"
      }
    ]
  }
]
"#
    );

    let _r = Result::<FieldElement, FieldElement>::Ok(FieldElement::ONE);
    // If the enum is generated, this will not work for enum impl conflict.
    // 29 |     let r = Result::<FieldElement, FieldElement>::Ok(FieldElement::ONE);
    //            ^^^^^^------------------------------ help: remove these generics
    //             |
    //             expected 0 generic arguments
}
