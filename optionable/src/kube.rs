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

use crate::Optionable;
use kube::Resource;
#[cfg(feature = "derive")]
#[doc(inline)]
pub use optionable_derive::optionable_kube;
#[cfg(feature = "derive")]
#[doc(inline)]
pub use optionable_derive::optionable_kube_cr;
#[cfg(feature = "derive")]
#[doc(inline)]
pub use optionable_derive::OptionableKubeCrd;
use serde::de::DeserializeOwned;
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use serde_json::{Error, Value};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::mem::take;

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

/// Prevent implementation outside of this crate.
trait ExtractManagedFieldsSealed {}

/// Trait to add the `extract(field_manager)` functionality to `kube::Resources + Optionable`.
/// Extracts the fields from the Resource `T` which are owned by the `field_manager`.
#[allow(private_bounds)]
pub trait ExtractManagedFields: ExtractManagedFieldsSealed + Optionable<Optioned: Sized> {
    /// Extracts the fields from the Resource `T` which are owned by the `field_manager`.
    /// The `metadata` entry is emptied except `name`, `namespace` and `generateName`.
    /// Returns `Ok(None)` is no fields at all are owned.
    ///
    /// # Errors
    /// - Serialization/Deserialization errors. The function uses `serde_json::Value` internally to get the
    ///   serialization keys referenced by Kubernetes.
    fn extract(self, field_manager: impl AsRef<str>) -> Result<Option<Self::Optioned>, Error>;
}

impl<T> ExtractManagedFieldsSealed for T
where
    T: Resource + Optionable + Serialize,
    T::Optioned: Sized + DeserializeOwned,
{
}

impl<T> ExtractManagedFields for T
where
    T: Resource + Optionable + Serialize,
    T::Optioned: Sized + DeserializeOwned,
{
    fn extract(mut self, field_manager: impl AsRef<str>) -> Result<Option<Self::Optioned>, Error> {
        // Managed fields are not forwarded to the result anyway so we can just take them.
        let managed_fields = take(&mut self.meta_mut().managed_fields);
        if let Some(managed_fields) = &managed_fields {
            let managed_fields = managed_fields.iter().find(|el| {
                el.fields_type
                    .as_ref()
                    .is_some_and(|fields_type| fields_type == "FieldsV1")
                    && el
                        .manager
                        .as_ref()
                        .is_some_and(|manager| manager == field_manager.as_ref())
            });
            if let Some(managed_fields) = managed_fields
                && let Some(fields) = &managed_fields.fields_v1
            {
                let mut data_json = serde_json::to_value(self)?;
                filter_json_value(&mut data_json, &fields.0, true);
                Ok(Some(serde_json::from_value::<T::Optioned>(data_json)?))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
/// Only retains those fields in `item` that are also contained in the `filter` json value.
/// Used by the Kubernetes server-side-apply `extract` implementations.
/// Has special handling for the `metadata` root entry whose `name`, `namespace` and `generateName` fields are copied over.
fn filter_json_value(item: &mut Value, filter: &Value, is_root: bool) {
    if let (Value::Object(item), Value::Object(filter)) = (item, filter) {
        let allowed_fields: HashMap<_, _> = filter
            .iter()
            .filter_map(|(k, v)| k.strip_prefix("f:").map(|s| (s.to_string(), v)))
            .collect();

        item.retain(|k, _| allowed_fields.contains_key(k) || (is_root && k == "metadata"));
        if is_root && let Some(meta) = item.get_mut("metadata") {
            filter_metadata(meta);
        }
        for (k, v) in item.iter_mut() {
            if let Some(filter_v) = filter.get(&format!("f:{k}")) {
                filter_json_value(v, filter_v, false);
            }
        }
    }
}

/// Only retains the fields `name`, `namespace` and `generateName`.
fn filter_metadata(item: &mut Value) {
    if let Value::Object(item) = item {
        item.retain(|k, _| k == "name" || k == "namespace" || k == "generateName");
    }
}

#[cfg(any(
    feature = "k8s_openapi_v1_30",
    feature = "k8s_openapi_v1_31",
    feature = "k8s_openapi_v1_32",
    feature = "k8s_openapi_v1_33",
    feature = "k8s_openapi_v1_34"
))]
#[cfg(test)]
mod test {
    use crate::kube::ExtractManagedFields;
    use k8s_openapi::api::apps::v1::{Deployment, DeploymentSpec};
    use k8s_openapi::api::core::v1::{Container, PodSpec, PodTemplateSpec};
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::{
        FieldsV1, ManagedFieldsEntry, ObjectMeta,
    };
    use serde_json::Value;

    #[test]
    fn extract_deployment() {
        let field_manager = "rust-manager";
        let managed_fields: Value =
            serde_json::from_str("{\"f:spec\":{\"f:replicas\": {}}}").unwrap();
        let deployment = Deployment {
            metadata: ObjectMeta {
                name: Some("my_name".to_owned()),
                namespace: Some("my_namespace".to_owned()),
                managed_fields: Some(vec![ManagedFieldsEntry {
                    fields_type: Some("FieldsV1".to_owned()),
                    fields_v1: Some(FieldsV1(managed_fields)),
                    manager: Some(field_manager.to_owned()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            spec: Some(DeploymentSpec {
                replicas: Some(2),
                template: PodTemplateSpec {
                    spec: Some(PodSpec {
                        containers: vec![Container {
                            name: "test".to_owned(),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        };
        let deployment_extract = deployment.clone().extract(field_manager).unwrap().unwrap();
        assert_eq!(deployment.metadata.name, deployment_extract.metadata.name);
        assert_eq!(
            deployment.metadata.namespace,
            deployment_extract.metadata.namespace
        );
        assert_eq!(
            deployment.spec.unwrap().replicas.unwrap(),
            deployment_extract.spec.as_ref().unwrap().replicas.unwrap()
        );
        assert_eq!(None, deployment_extract.spec.unwrap().template);
    }
}
