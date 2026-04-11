#![cfg(any(feature = "alloc", feature = "std"))]

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

fn get_targets() -> Vec<EnvVar> {
    vec![
        EnvVar {
            name: "el1".to_owned(),
            value: "val1".to_owned(),
        },
        EnvVar {
            name: "el2".to_owned(),
            value: "val2".to_owned(),
        },
    ]
}
fn get_others() -> Vec<EnvVarOpt> {
    vec![
        EnvVarOpt {
            name: "el3".to_owned(),
            value: Some("val1".to_owned()),
        },
        EnvVarOpt {
            name: "el2".to_owned(),
            value: Some("val4".to_owned()),
        },
    ]
}

#[test]
fn merge_map_no_wrapper() {
    let mut targets = get_targets();
    try_merge_optioned_map(&mut targets, get_others()).unwrap();
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

#[test]
fn merge_optionable_convert() {
    use optionable::OptionableConvert;

    #[derive(Optionable, PartialEq, Debug)]
    struct Wrapper {
        #[optionable(merge(optionable_convert))]
        items: Vec<EnvVar>,
    }

    let mut targets = Wrapper {
        items: get_targets(),
    };
    let others = WrapperOpt {
        items: Some(get_others()),
    };
    targets.merge(others).unwrap();
    assert_eq!(
        targets,
        Wrapper {
            items: vec![
                EnvVar {
                    name: "el3".to_owned(),
                    value: "val1".to_owned(),
                },
                EnvVar {
                    name: "el2".to_owned(),
                    value: "val4".to_owned(),
                },
            ]
        }
    );
}

#[test]
fn merge_atomic() {
    use optionable::OptionableConvert;

    #[derive(Optionable, PartialEq, Debug)]
    struct Wrapper {
        #[optionable(merge(atomic))]
        items: Vec<EnvVar>,
    }

    let mut targets = Wrapper {
        items: get_targets(),
    };
    let others = WrapperOpt {
        items: Some(get_others()),
    };
    targets.merge(others).unwrap();
    assert_eq!(
        targets,
        Wrapper {
            items: vec![
                EnvVar {
                    name: "el3".to_owned(),
                    value: "val1".to_owned(),
                },
                EnvVar {
                    name: "el2".to_owned(),
                    value: "val4".to_owned(),
                },
            ]
        }
    );
}

#[test]
fn merge_set() {
    use optionable::OptionableConvert;

    #[derive(Optionable, PartialEq, Debug)]
    struct Wrapper {
        #[optionable(merge(set))]
        items: Vec<EnvVar>,
    }

    let mut targets = Wrapper {
        items: get_targets(),
    };
    let others = WrapperOpt {
        items: Some(get_others()),
    };
    targets.merge(others).unwrap();
    assert_eq!(
        targets,
        Wrapper {
            items: vec![
                EnvVar {
                    name: "el1".to_owned(),
                    value: "val1".to_owned(),
                },
                EnvVar {
                    name: "el2".to_owned(),
                    value: "val2".to_owned(),
                },
                EnvVar {
                    name: "el3".to_owned(),
                    value: "val1".to_owned(),
                },
                EnvVar {
                    name: "el2".to_owned(),
                    value: "val4".to_owned(),
                },
            ]
        }
    );
}

#[test]
fn merge_map() {
    use optionable::OptionableConvert;

    #[derive(Optionable, PartialEq, Debug)]
    struct Wrapper {
        #[optionable(merge(map))]
        items: Vec<EnvVar>,
    }

    let mut targets = Wrapper {
        items: get_targets(),
    };
    let others = WrapperOpt {
        items: Some(get_others()),
    };
    targets.merge(others).unwrap();
    assert_eq!(
        targets,
        Wrapper {
            items: vec![
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
        }
    );
}
