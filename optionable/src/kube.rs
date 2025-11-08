//! Tooling to derive optioned types for `kube::CustomResource`.
//!
//! The standard usage would be to add the `#[optionable_kube_cr]` to the spec type
//! used to define the `kube::CustomResource` and the `#[optionable_kube]` attribute macro
//! to all custom types used as subfields. The `#[optionable_kube_cr]` attribute macro
//! must be placed prior to any #[derive(...)] statements for the given type.
//!
//! For more specialized use cases the documentation of these attribute macros shows to
//! what they resolve for customization.
//!
//! ```rust,ignore
//! #[optionable_kube_cr]
//! #[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
//! #[kube(group = "example.localhost", version = "v1", kind = "MyCrd", namespaced)]
//! pub struct MyCrdSpec {
//!     pub msg: String,
//!     pub template: MyCrdSpecTemplate,
//! }
//!
//! #[optionable_kube]
//! #[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
//! pub struct MyCrdSpecTemplate {
//!     pub replicas: u32,
//! }
//! ```

use kube::Resource;
use serde::ser::SerializeMap;
use serde::Serializer;
use std::marker::PhantomData;

#[cfg(feature = "derive")]
#[doc(inline)]
pub use optionable_derive::optionable_kube;
#[cfg(feature = "derive")]
#[doc(inline)]
pub use optionable_derive::optionable_kube_cr;
#[cfg(feature = "derive")]
#[doc(inline)]
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
