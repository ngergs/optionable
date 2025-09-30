use optionable::OptionedConvert;
use optionable::{Optionable, OptionableConvert};
use serde::Deserialize;
use serde::Serialize;

#[test]
/// Check that the derive macro works with visibility modifier.
fn derive_named_struct() {
    #[derive(Optionable)]
    #[optionable(derive(Clone))]
    struct DeriveExample {
        name: String,
        pub surname: String,
    }

    let _ = DeriveExampleOpt {
        name: None,
        surname: None,
    };
    let partial = DeriveExampleOpt {
        name: None,
        surname: Some("c".to_owned()),
    };
    let full_partial = DeriveExampleOpt {
        name: Some("a".to_owned()),
        surname: Some("b".to_owned()),
    };
    let mut full: DeriveExample = full_partial.clone().try_into_optionable().unwrap();
    assert_eq!(full.name, full_partial.name.clone().unwrap());
    assert_eq!(full.surname, full_partial.surname.unwrap());
    full.merge(partial.clone()).unwrap();
    assert_eq!(full.name, full_partial.name.unwrap());
    assert_eq!(full.surname, partial.surname.unwrap());
}

#[test]
/// Named struct with a required field
fn derive_named_struct_required() {
    #[derive(Optionable)]
    #[optionable(derive(Clone))]
    #[allow(dead_code)]
    struct DeriveExample {
        name: String,
        #[optionable(required)]
        surname: String,
    }

    let partial = DeriveExampleOpt {
        name: None,
        surname: "c".to_owned(),
    };
    let full_partial = DeriveExampleOpt {
        name: Some("a".to_owned()),
        surname: "b".to_owned(),
    };

    let mut full: DeriveExample = full_partial.clone().try_into_optionable().unwrap();
    assert_eq!(full_partial.clone().name.unwrap(), full.name);
    assert_eq!(full_partial.surname, full.surname);
    full.merge(partial.clone()).unwrap();
    assert_eq!(full_partial.name.unwrap(), full.name);
    assert_eq!(partial.surname, full.surname);
}

#[test]
/// Tuple with visibility modifiers
fn derive_unnamed_struct() {
    #[allow(dead_code)]
    #[derive(Optionable)]
    #[optionable(derive(Clone))]
    struct DeriveExample(String, pub i32);

    let partial = DeriveExampleOpt(None, Some(1));
    let full_partial = DeriveExampleOpt(Some("a".to_owned()), Some(42));

    let mut full: DeriveExample = full_partial.clone().try_into_optionable().unwrap();
    assert_eq!(full_partial.clone().0.unwrap(), full.0);
    assert_eq!(full_partial.1.unwrap(), full.1);
    full.merge(partial.clone()).unwrap();
    assert_eq!(full_partial.0.unwrap(), full.0);
    assert_eq!(partial.1.unwrap(), full.1);
}

#[test]
/// Tuple with a required field
fn derive_unnamed_struct_required() {
    #[derive(Optionable)]
    #[allow(dead_code)]
    #[optionable(derive(Clone))]
    struct DeriveExample(String, #[optionable(required)] i32);

    let partial = DeriveExampleOpt(None, 1);
    let full_partial = DeriveExampleOpt(Some("a".to_owned()), 42);

    let mut full: DeriveExample = full_partial.clone().try_into_optionable().unwrap();
    assert_eq!(full_partial.clone().0.unwrap(), full.0);
    assert_eq!(full_partial.1, full.1);
    full.merge(partial.clone()).unwrap();
    assert_eq!(full_partial.0.unwrap(), full.0);
    assert_eq!(partial.1, full.1);
}

#[test]
/// Check that the derive macro works.
fn derive_generic() {
    #[allow(dead_code)]
    #[derive(Optionable)]
    #[optionable(derive(Clone))]
    struct DeriveExample<T, T2>
    where
        T: Optionable,
        T2: Optionable,
        T::Optioned: Clone,
        T2::Optioned: Clone,
    {
        name: T,
        surname: T2,
    }

    let partial = DeriveExampleOpt::<i32, String> {
        name: Some(42),
        surname: None,
    };
    let full_partial = DeriveExampleOpt::<i32, String> {
        name: Some(2),
        surname: Some("b".to_owned()),
    };

    let mut full: DeriveExample<_, _> = full_partial.clone().try_into_optionable().unwrap();
    assert_eq!(full_partial.clone().name.unwrap(), full.name);
    assert_eq!(full_partial.surname.clone().unwrap(), full.surname);
    full.merge(partial.clone()).unwrap();
    assert_eq!(partial.name.unwrap(), full.name);
    assert_eq!(full_partial.surname.unwrap(), full.surname);
}

type _String = <String as ::optionable::Optionable>::Optioned;

#[test]
/// Check that the derive macro works with nested structs
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
/// Check that the derive macro works for enums
fn derive_enum() {
    #[allow(dead_code)]
    #[derive(Optionable)]
    enum DeriveExample {
        Unit,
        Plain(String),
        Address { street: String, number: u32 },
        AddressTuple(String, u32),
    }

    // unit does not get optioned as it has no inner structure
    let _ = DeriveExampleOpt::Unit;

    let _ = DeriveExampleOpt::Plain(None);
    let _ = DeriveExampleOpt::Address {
        street: None,
        number: None,
    };
    let _ = DeriveExampleOpt::AddressTuple(None, None);

    let plain_optioned = DeriveExampleOpt::Plain(Some("a".to_owned()));
    let plain: DeriveExample = plain_optioned.try_into_optionable().unwrap();
    if let DeriveExample::Plain(val) = plain {
        assert_eq!(val, "a".to_owned());
    } else {
        panic!("optioned variant does not coincide with expected");
    }

    let address_optioned = DeriveExampleOpt::Address {
        street: Some("a".to_owned()),
        number: Some(42),
    };
    let mut address: DeriveExample = address_optioned.try_into_optionable().unwrap();
    if let DeriveExample::Address { street, number } = &address {
        assert_eq!(*street, "a".to_owned());
        assert_eq!(*number, 42);
    } else {
        panic!("optioned variant does not coincide with expected");
    }
    let address_patch = DeriveExampleOpt::Address {
        street: Some("b".to_owned()),
        number: None,
    };
    address.merge(address_patch).unwrap();
    if let DeriveExample::Address { street, number } = &address {
        assert_eq!(*street, "b".to_owned());
        assert_eq!(*number, 42);
    } else {
        panic!("optioned variant does not coincide with expected");
    }
    let address2_optioned = DeriveExampleOpt::AddressTuple(Some("a".to_owned()), Some(42));
    let address2: DeriveExample = address2_optioned.try_into_optionable().unwrap();
    if let DeriveExample::AddressTuple(street, number) = address2 {
        assert_eq!(street, "a".to_owned());
        assert_eq!(number, 42);
    } else {
        panic!("optioned variant does not coincide with expected");
    }
}

#[test]
/// Check that forwarding other derives via helper attributes works
fn derive_forward_other_derives() {
    #[derive(Optionable)]
    #[optionable(derive(Deserialize, Serialize))]
    #[allow(dead_code)]
    struct DeriveExample {
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
    assert_eq!(a_json, "{\"name\":\"a\",\"middle_name\":\"b\"}");
}
