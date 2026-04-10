#![cfg(feature = "std")]

use optionable::{
    Optionable,
    merge::{MapKeysEq, try_merge_optioned_map},
};

#[derive(Optionable, Debug, PartialEq)]
struct EnvVar {
    #[optionable(required)]
    name: String,
    value: String,
}

impl MapKeysEq for EnvVar {
    fn keys_eq(&self, other: &Self::Optioned) -> bool {
        self.name == other.name
    }
}

#[test]
fn merge_map() {
    let mut targets = vec![
        EnvVar {
            name: "el1".to_owned(),
            value: "val1".to_owned(),
        },
        EnvVar {
            name: "el2".to_owned(),
            value: "val2".to_owned(),
        },
    ];
    let others = vec![
        EnvVarOpt {
            name: "el3".to_owned(),
            value: Some("val1".to_owned()),
        },
        EnvVarOpt {
            name: "el2".to_owned(),
            value: Some("val4".to_owned()),
        },
    ];
    try_merge_optioned_map(&mut targets, others).unwrap();
    assert_eq!(
        targets,
        vec![
            EnvVar {
                name: "el1".to_owned(),
                value: "val1".to_owned(),
            },
            EnvVar {
                name: "el2".to_owned(),
                value: "val4".to_owned(),
            },
            EnvVar {
                name: "el3".to_owned(),
                value: "val1".to_owned(),
            },
        ]
    );
}
