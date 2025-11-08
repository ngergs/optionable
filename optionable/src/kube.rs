use kube::Resource;
use serde::ser::SerializeMap;
use serde::Serializer;
use std::marker::PhantomData;

pub use optionable_derive::optionable_kube;
pub use optionable_derive::optionable_kube_cr;
pub use optionable_derive::OptionableKubeCrd;

/// Serializes a `PhantomData` marker to add the API envelope fields `apiVersion` and `kind`.
/// Intended use is together with `#[serde(flatten)]` for the marker field.
///
/// # Errors
/// - Forwards any serialization errors.
pub fn serialize_api_envelope<S: Serializer, R: Resource<DynamicType = ()>>(
    _: &PhantomData<R>,
    s: S,
) -> Result<S::Ok, S::Error> {
    let mut map = s.serialize_map(Some(2))?;
    map.serialize_entry("apiVersion", R::api_version(&()).as_ref())?;
    map.serialize_entry("kind", R::kind(&()).as_ref())?;
    map.end()
}
