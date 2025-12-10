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
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serializer};
use std::collections::HashMap;
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

/// Serializes a `PhantomData` marker to add the API envelope fields `apiVersion` and `kind`.
/// Intended use is together with `#[serde(flatten)]` for the marker field.
pub fn serialize_api_envelope<S: Serializer, R: Resource>(
    _: &PhantomData<R>,
    s: S,
) -> Result<S::Ok, S::Error> {
    let mut map = s.serialize_map(Some(2))?;
    map.serialize_entry("apiVersion", R::API_VERSION)?;
    map.serialize_entry("kind", R::KIND)?;
    map.end()
}

/// Deserializes a `PhantomData` marker to verify the API envelope fields `apiVersion` and `kind`.
/// Intended use is together with `#[serde(flatten)]` for the marker field.
///
/// # Errors
/// - If the marker fields do not have the expected value specified in the `Resource` wrapped by the `PhantomData`.
/// - Forwards any deserialization errors.
pub fn deserialize_api_envelope<'de, D: Deserializer<'de>, R: Resource>(
    d: D,
) -> Result<PhantomData<R>, D::Error> {
    let envelope: HashMap<String, String> = HashMap::deserialize(d)?;

    if let Some(api_version) = envelope.get("apiVersion") {
        let api_version_expected = R::API_VERSION;
        if api_version != api_version_expected {
            return Err(Error::invalid_value(
                Unexpected::Str(api_version),
                &format!("apiVersion: {api_version_expected}").as_str(),
            ));
        }
    } else {
        return Err(Error::missing_field("apiVersion"));
    }
    if let Some(kind) = envelope.get("kind") {
        let kind_expected = R::KIND;
        if kind != kind_expected {
            return Err(Error::invalid_value(
                Unexpected::Str(kind),
                &format!("kind: {kind_expected}").as_str(),
            ));
        }
    } else {
        return Err(Error::missing_field("kind"));
    }
    Ok(PhantomData)
}

#[cfg(test)]
mod test {
    use crate::k8s_openapi::apimachinery::pkg::util::intstr::IntOrStringAc;
    use k8s_openapi::api::apps::v1::Deployment;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::marker::PhantomData;

    #[derive(Serialize, Deserialize, Default, Debug)]
    struct ApiEnvelopeDeployment {
        #[serde(flatten)]
        #[serde(
            serialize_with = "crate::k8s_openapi::serialize_api_envelope",
            deserialize_with = "crate::k8s_openapi::deserialize_api_envelope"
        )]
        phantom: PhantomData<Deployment>,
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
}
