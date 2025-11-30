#![cfg(any(feature = "alloc", feature = "std"))]

use optionable_derive::Optionable;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[test]
/// Check that the derive macro works with nested structs and container types like `Vec`
fn derive_nested() {
    #[derive(Optionable)]
    #[allow(dead_code)]
    struct DeriveExample {
        name: String,
        addresses: Vec<Address>,
    }
    #[derive(Optionable)]
    #[allow(dead_code)]
    struct Address {
        street_name: String,
        number: u8,
    }

    let _ = DeriveExampleOpt {
        name: None,
        addresses: None,
    };
    let _ = DeriveExampleOpt {
        name: Some("a".to_owned()),
        addresses: Some(vec![AddressOpt {
            street_name: None,
            number: None,
        }]),
    };
    let _ = DeriveExampleOpt {
        name: Some("a".to_owned()),
        addresses: Some(vec![AddressOpt {
            street_name: Some("B".to_owned()),
            number: Some(2),
        }]),
    };
    let _ = DeriveExampleOpt {
        name: None,
        addresses: None,
    };
}

#[test]
/// Check that forwarding other derives via helper attributes works
fn derive_forward_other_derives() {
    #[derive(Optionable)]
    #[optionable(derive(Deserialize, Serialize, Default))]
    #[optionable_attr(serde(rename_all = "camelCase", deny_unknown_fields))]
    #[optionable_attr(serde(default))]
    #[allow(dead_code)]
    struct DeriveExample {
        #[optionable_attr(serde(rename = "firstName"))]
        name: String,
        middle_name: Option<String>,
        surname: String,
    }

    let a = DeriveExampleOpt {
        name: Some("a".to_owned()),
        middle_name: Some("b".to_owned()), // no nested Options here
        surname: None,
    };
    let a_json = serde_json::to_string(&a).unwrap();
    assert_eq!(a_json, "{\"firstName\":\"a\",\"middleName\":\"b\"}");
}

#[test]
/// Check that the derive macro works for a bunch of not plain types.
fn derive_generic_advanced_types() {
    #[allow(dead_code)]
    #[derive(Optionable)]
    #[optionable(derive(Clone, Serialize, Deserialize))]
    struct DeriveExample<T, T2, T3>
    where
        T: Debug,
    {
        array: [T; 3],
        tuple: (T, T2, T3),
    }

    let _ = DeriveExampleOpt::<u32, String, i32> {
        array: Some([0, 1, 2]),
        tuple: Some((Some(2), Some("a".to_owned()), Some(-1))),
    };
}
