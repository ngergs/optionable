#![allow(clippy::all)]
#![allow(clippy::pedantic)]

use k8s_openapi::Resource;
use serde::ser::SerializeMap;
use serde::Serializer;
use std::marker::PhantomData;

mod optionable;
#[cfg(feature = "k8s_openapi_v1_30")]
pub mod v1_30;
#[cfg(feature = "k8s_openapi_v1_31")]
pub mod v1_31;
#[cfg(feature = "k8s_openapi_v1_32")]
pub mod v1_32;
#[cfg(feature = "k8s_openapi_v1_33")]
pub mod v1_33;
#[cfg(feature = "k8s_openapi_v1_34")]
pub mod v1_34;

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

#[cfg(test)]
mod test {
    #[cfg(feature = "k8s_openapi_v1_30")]
    use crate::k8s_openapi::v1_30::apimachinery::pkg::util::intstr::IntOrStringAc;
    #[cfg(feature = "k8s_openapi_v1_31")]
    use crate::k8s_openapi::v1_31::apimachinery::pkg::util::intstr::IntOrStringAc;
    #[cfg(feature = "k8s_openapi_v1_32")]
    use crate::k8s_openapi::v1_32::apimachinery::pkg::util::intstr::IntOrStringAc;
    #[cfg(feature = "k8s_openapi_v1_33")]
    use crate::k8s_openapi::v1_33::apimachinery::pkg::util::intstr::IntOrStringAc;
    #[cfg(feature = "k8s_openapi_v1_34")]
    use crate::k8s_openapi::v1_34::apimachinery::pkg::util::intstr::IntOrStringAc;

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
