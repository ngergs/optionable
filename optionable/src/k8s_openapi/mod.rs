#![allow(clippy::all)]
#![allow(clippy::pedantic)]

//! # k8s-openapi
//! Generated optioned types and implementations of the `Optionable`-trait for types from [`k8s-openapi`](https://github.com/Arnavion/k8s-openapi).
//!
//! ## Limitations
//!
//! [WatchEvent](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.34/#watchevent-v1-meta) is a very special
//! case as it is an [externally tagged enum](https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_34/apimachinery/pkg/apis/meta/v1/watch_event.rs)
//! where two variants share a tag `ERROR`. There is no obvious way how to model this without using a custom serialization/deserialization.
//! As a WatchEvent is usually only consumed and not patched there is likely not a use case for having
//! an optioned WatchEvent so it is dropped for now.

use k8s_openapi::Resource;
use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer, Serializer};
use std::marker::PhantomData;

mod optionable;

#[cfg(feature = "k8s_openapi_v1_30")]
mod v1_30;
#[cfg(feature = "k8s_openapi_v1_30")]
pub use v1_30::*;
#[cfg(feature = "k8s_openapi_v1_31")]
mod v1_31;
#[cfg(feature = "k8s_openapi_v1_31")]
pub use v1_31::*;
#[cfg(feature = "k8s_openapi_v1_32")]
mod v1_32;
#[cfg(feature = "k8s_openapi_v1_32")]
pub use v1_32::*;
#[cfg(feature = "k8s_openapi_v1_33")]
mod v1_33;
#[cfg(feature = "k8s_openapi_v1_33")]
pub use v1_33::*;
#[cfg(feature = "k8s_openapi_v1_34")]
mod v1_34;
#[cfg(feature = "k8s_openapi_v1_34")]
pub use v1_34::*;

/// Serializes a `PhantomData` marker to add the API envelope field content for `apiVersion`.
/// Intended to be used with `apiVersion: PhantomData<T>`.
///
/// # Errors
/// - forwards any serialization errors.
pub fn serialize_api_version<S: Serializer, R: Resource>(
    _: &PhantomData<R>,
    s: S,
) -> Result<S::Ok, S::Error> {
    s.serialize_str(R::API_VERSION)
}

/// Serializes a `PhantomData` marker to add the API envelope field content for `kind`.
/// Intended to be used with `kind: PhantomData<T>`.
///
/// # Errors
/// - forwards any serialization errors.
pub fn serialize_kind<S: Serializer, R: Resource>(
    _: &PhantomData<R>,
    s: S,
) -> Result<S::Ok, S::Error> {
    s.serialize_str(R::KIND)
}

/// Deserializes a `PhantomData` marker to verify the API envelope content for `apiVersion`.
/// Intended to be used with `apiVersion: PhantomData<T>`.
///
/// # Errors
/// - If the marker field do not have the expected value specified in the `Resource` wrapped by the `PhantomData`.
/// - Forwards any deserialization errors.
pub fn deserialize_api_version<'de, D: Deserializer<'de>, R: Resource>(
    d: D,
) -> Result<PhantomData<R>, D::Error> {
    let val = String::deserialize(d)?;
    if val != R::API_VERSION {
        return Err(Error::invalid_value(
            Unexpected::Str(&val),
            &format!("apiVersion: {}", R::API_VERSION).as_str(),
        ));
    }
    Ok(PhantomData)
}

/// Deserializes a `PhantomData` marker to verify the API envelope content for `apiVersion`.
/// Intended to be used with `apiVersion: PhantomData<T>`.
///
/// # Errors
/// - If the marker field do not have the expected value specified in the `Resource` wrapped by the `PhantomData`.
/// - Forwards any deserialization errors.
pub fn deserialize_kind<'de, D: Deserializer<'de>, R: Resource>(
    d: D,
) -> Result<PhantomData<R>, D::Error> {
    let val = String::deserialize(d)?;
    if val != R::KIND {
        return Err(Error::invalid_value(
            Unexpected::Str(&val),
            &format!("kind: {}", R::KIND).as_str(),
        ));
    }
    Ok(PhantomData)
}

#[cfg(test)]
mod test {
    use crate::k8s_openapi::apimachinery::pkg::api::resource::QuantityAc;
    use crate::k8s_openapi::apimachinery::pkg::util::intstr::IntOrStringAc;
    use k8s_openapi::api::apps::v1::Deployment;
    use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::marker::PhantomData;

    #[derive(Serialize, Deserialize, Default, Debug)]
    #[serde(rename_all = "camelCase")]
    struct ApiEnvelopeDeployment {
        #[serde(
            serialize_with = "crate::k8s_openapi::serialize_api_version",
            deserialize_with = "crate::k8s_openapi::deserialize_api_version"
        )]
        api_version: PhantomData<Deployment>,
        #[serde(
            serialize_with = "crate::k8s_openapi::serialize_kind",
            deserialize_with = "crate::k8s_openapi::deserialize_kind"
        )]
        kind: PhantomData<Deployment>,
    }

    #[test]
    fn serialize_api_envelope() {
        let envelope = ApiEnvelopeDeployment {
            ..Default::default()
        };
        let json = serde_json::to_value(envelope).unwrap();
        assert_eq!(json, json!({"apiVersion": "apps/v1", "kind": "Deployment"}));
    }

    #[test]
    fn deserialize_api_envelope_happy() {
        let json = json!({"apiVersion": "apps/v1", "kind": "Deployment"});
        let _: ApiEnvelopeDeployment = serde_json::from_value(json).unwrap();
    }

    #[test]
    fn deserialize_api_envelope_typo() {
        let json = json!({"apiVersion": "apps/v1", "kind": "Deployments"});
        let err = serde_json::from_value::<ApiEnvelopeDeployment>(json).unwrap_err();
        assert_eq!(
            err.to_string(),
            "invalid value: string \"Deployments\", expected kind: Deployment"
        );
    }

    /// Tests if deserialization for `IntOrString` works.
    #[test]
    fn int_or_string_deser() {
        struct TestCase {
            input: &'static str,
            expected: Result<IntOrStringAc, String>,
        }
        let tcs = [
            TestCase {
                input: "\"v1\"",
                expected: Ok(IntOrStringAc::String(Some("v1".to_owned()))),
            },
            TestCase {
                input: "42",
                expected: Ok(IntOrStringAc::Int(Some(42))),
            },
            TestCase {
                input: "-42",
                expected: Ok(IntOrStringAc::Int(Some(-42))),
            },
            TestCase {
                input: "\"25%\"",
                expected: Ok(IntOrStringAc::String(Some("25%".to_owned()))),
            },
        ];
        for tc in tcs.into_iter() {
            let result: Result<IntOrStringAc, _> =
                serde_json::from_str(tc.input).map_err(|e| e.to_string());
            assert_eq!(tc.expected, result);
        }
    }

    #[test]
    /// Quantity has a specialized serialization handling that allows to deserialize the
    /// inner string from a string or an int (serialization is always a string).
    /// Our optioned type should mimic this.
    fn quantity_deser() {
        let datas = vec![json!("42"), json!(42)];
        for data in datas {
            let val = serde_json::from_value::<Quantity>(data.clone()).unwrap();
            assert_eq!(val, Quantity("42".to_owned()));
            let val_ac = serde_json::from_value::<QuantityAc>(data).unwrap();
            assert_eq!(val_ac, QuantityAc(Some("42".to_owned())));
        }
    }
}
