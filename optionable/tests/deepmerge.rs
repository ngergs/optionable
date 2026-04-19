#![cfg(feature = "kube3")] // implicitly brints k8s_openapi in scope

use k8s_openapi027::DeepMerge;
use optionable::DeepMerge;

#[derive(DeepMerge, PartialEq, Debug)]
#[deepmerge(crate_k8s_openapi = "k8s_openapi027")]
struct EnvVar {
    key: String,
    value: Option<String>,
}

#[test]
fn deepmerge() {
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
