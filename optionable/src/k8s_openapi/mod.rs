#![allow(clippy::all)]
#![allow(clippy::pedantic)]

//todo: revert
pub use k8s_openapi;

use k8s_openapi::Resource;
use serde::ser::SerializeMap;
use serde::Serializer;
use std::marker::PhantomData;

mod optionable;
#[cfg(feature = "k8s_openapi_v1_34")]
pub mod v1_34;

/// Serializes a PhantomData marker to add the API envelope fields `apiVersion` and `kind`.
/// Intended use is together with `#[serde(flatten)]` for the marker field.
pub(crate) fn serialize_api_envelope<S: Serializer, R: Resource>(
    _: &PhantomData<R>,
    s: S,
) -> Result<S::Ok, S::Error> {
    let mut map = s.serialize_map(Some(2))?;
    map.serialize_entry("apiVersion", R::API_VERSION)?;
    map.serialize_entry("kind", R::KIND)?;
    map.end()
}
