#![cfg(all(feature = "kube3", feature = "k8s_openapi027_v1_34"))] // implicitly brints k8s_openapi in scope

use k8s_openapi027::DeepMerge;
use optionable::DeepMerge;
use optionable_derive::MapKeysEq;

#[derive(DeepMerge, MapKeysEq, PartialEq, Debug, Clone)]
#[deepmerge(crate_k8s_openapi = "k8s_openapi027")]
struct EnvVar {
    #[map_key]
    key: String,
    value: Option<String>,
}

#[test]
fn deepmerge_struct() {
    let mut val = EnvVar {
        key: "hello".to_owned(),
        value: Some("world".to_owned()),
    };
    val.merge_from(EnvVar {
        key: "hello2".to_owned(),
        value: None,
    });
    assert_eq!(
        &val,
        &EnvVar {
            key: "hello2".to_owned(),
            value: Some("world".to_owned()),
        }
    );
    val.merge_from(EnvVar {
        key: "hello2".to_owned(),
        value: Some("world2".to_owned()),
    });
    assert_eq!(
        &val,
        &EnvVar {
            key: "hello2".to_owned(),
            value: Some("world2".to_owned()),
        }
    );
}

#[derive(DeepMerge, PartialEq, Debug, Clone)]
#[deepmerge(crate_k8s_openapi = "k8s_openapi027")]
enum EnvVarEnum {
    KeyOnly(String),
    Full { key: String, value: String },
}

#[test]
fn deepmerge_enum() {
    let mut val = EnvVarEnum::KeyOnly("hi".to_owned());
    val.merge_from(EnvVarEnum::KeyOnly("hello".to_owned()));
    assert_eq!(&val, &EnvVarEnum::KeyOnly("hello".to_owned()));

    let other = EnvVarEnum::Full {
        key: "hello".to_owned(),
        value: "world".to_owned(),
    };
    val.merge_from(other.clone());
    assert_eq!(val, other);
}

#[derive(DeepMerge, PartialEq, Debug, Clone)]
#[deepmerge(crate_k8s_openapi = "k8s_openapi027")]
/// just verify that it generates valid code
#[allow(dead_code)]
struct AllMethods {
    #[deepmerge(method(atomic))]
    atomic: Vec<EnvVar>,
    #[deepmerge(method(append_not_present))]
    append_not_present: Vec<EnvVar>,
    #[deepmerge(method(iter_map))]
    iter_map: Vec<EnvVar>,
}

#[test]
fn deepmerge_mapping() {
    let envvar_init = vec![
        EnvVar {
            key: "hello".to_owned(),
            value: Some("hello".to_owned()),
        },
        EnvVar {
            key: "hello2".to_owned(),
            value: Some("hello2".to_owned()),
        },
        EnvVar {
            key: "hello3".to_owned(),
            value: None,
        },
        EnvVar {
            key: "hello4".to_owned(),
            value: Some("world".to_owned()),
        },
    ];
    let envvar_other = vec![
        EnvVar {
            key: "hello".to_owned(),
            value: Some("world".to_owned()),
        },
        EnvVar {
            key: "hello2".to_owned(),
            value: None,
        },
        EnvVar {
            key: "hello3".to_owned(),
            value: Some("world".to_owned()),
        },
        EnvVar {
            key: "hello4".to_owned(),
            value: Some("world".to_owned()),
        },
    ];
    let wrapper = AllMethods {
        atomic: envvar_init.clone(),
        append_not_present: envvar_init.clone(),
        iter_map: envvar_init,
    };
    let wrapper_other = AllMethods {
        atomic: envvar_other.clone(),
        append_not_present: envvar_other.clone(),
        iter_map: envvar_other.clone(),
    };
    let mut wrapper_merged = wrapper.clone();
    wrapper_merged.merge_from(wrapper_other);
    let wrapper_expected = AllMethods {
        atomic: envvar_other,
        append_not_present: vec![
            EnvVar {
                key: "hello".to_owned(),
                value: Some("hello".to_owned()),
            },
            EnvVar {
                key: "hello2".to_owned(),
                value: Some("hello2".to_owned()),
            },
            EnvVar {
                key: "hello3".to_owned(),
                value: None,
            },
            EnvVar {
                key: "hello4".to_owned(),
                value: Some("world".to_owned()),
            },
            EnvVar {
                key: "hello".to_owned(),
                value: Some("world".to_owned()),
            },
            EnvVar {
                key: "hello2".to_owned(),
                value: None,
            },
            EnvVar {
                key: "hello3".to_owned(),
                value: Some("world".to_owned()),
            },
        ],
        iter_map: vec![
            EnvVar {
                key: "hello".to_owned(),
                value: Some("world".to_owned()),
            },
            EnvVar {
                key: "hello2".to_owned(),
                value: Some("hello2".to_owned()),
            },
            EnvVar {
                key: "hello3".to_owned(),
                value: Some("world".to_owned()),
            },
            EnvVar {
                key: "hello4".to_owned(),
                value: Some("world".to_owned()),
            },
        ],
    };
    assert_eq!(wrapper_merged, wrapper_expected);
}
