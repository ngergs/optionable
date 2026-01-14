//! Tooling to help deriving optioned types for `kube::CustomResource`.

use crate::Optionable;
use kube3::Resource;
use serde::de::{DeserializeOwned, Error, Unexpected};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::mem::take;

/// Serializes a `PhantomData` marker to add the API envelope field content for `apiVersion`.
/// Intended to be used with `apiVersion: PhantomData<T>`.
///
/// # Errors
/// - forwards any serialization errors.
pub fn serialize_api_version<S: Serializer, R: Resource<DynamicType = ()>>(
    _: &PhantomData<R>,
    s: S,
) -> Result<S::Ok, S::Error> {
    s.serialize_str(R::api_version(&()).as_ref())
}

/// Serializes a `PhantomData` marker to add the API envelope field content for `kind`.
/// Intended to be used with `kind: PhantomData<T>`.
///
/// # Errors
/// - forwards any serialization errors.
pub fn serialize_kind<S: Serializer, R: Resource<DynamicType = ()>>(
    _: &PhantomData<R>,
    s: S,
) -> Result<S::Ok, S::Error> {
    s.serialize_str(R::kind(&()).as_ref())
}

/// Deserializes a `PhantomData` marker to verify the API envelope content for `apiVersion`.
/// Intended to be used with `apiVersion: PhantomData<T>`.
///
/// # Errors
/// - If the marker field do not have the expected value specified in the `Resource` wrapped by the `PhantomData`.
/// - Forwards any deserialization errors.
pub fn deserialize_api_version<'de, D: Deserializer<'de>, R: Resource<DynamicType = ()>>(
    d: D,
) -> Result<PhantomData<R>, D::Error> {
    let val = String::deserialize(d)?;
    if val != R::api_version(&()).as_ref() {
        return Err(Error::invalid_value(
            Unexpected::Str(&val),
            &format!("apiVersion: {}", R::api_version(&()).as_ref()).as_str(),
        ));
    }
    Ok(PhantomData)
}

/// Deserializes a `PhantomData` marker to verify the API envelope content for `kind`.
/// Intended to be used with `kind: PhantomData<T>`.
///
/// # Errors
/// - If the marker field do not have the expected value specified in the `Resource` wrapped by the `PhantomData`.
/// - Forwards any deserialization errors.
pub fn deserialize_kind<'de, D: Deserializer<'de>, R: Resource<DynamicType = ()>>(
    d: D,
) -> Result<PhantomData<R>, D::Error> {
    let val = String::deserialize(d)?;
    if val != R::kind(&()).as_ref() {
        return Err(Error::invalid_value(
            Unexpected::Str(&val),
            &format!("kind: {}", R::kind(&()).as_ref()).as_str(),
        ));
    }
    Ok(PhantomData)
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
    fn extract(self, field_manager: &str) -> Result<Option<Self::Optioned>, serde_json::Error>;
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
    fn extract(mut self, field_manager: &str) -> Result<Option<Self::Optioned>, serde_json::Error> {
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
                        .is_some_and(|manager| manager == field_manager)
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
/// Also copies over `apiVersion` and `kind` root entries.
fn filter_json_value(item: &mut Value, filter: &Value, is_root: bool) {
    if let (Value::Object(item), Value::Object(filter)) = (item, filter) {
        let allowed_fields: HashMap<_, _> = filter
            .iter()
            .filter_map(|(k, v)| k.strip_prefix("f:").map(|s| (s, v)))
            .collect();

        item.retain(|k, _| {
            allowed_fields.contains_key(k.as_str())
                || (is_root && (k == "apiVersion" || k == "kind" || k == "metadata"))
        });
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
    feature = "k8s_openapi027_v1_31",
    feature = "k8s_openapi027_v1_32",
    feature = "k8s_openapi027_v1_33",
    feature = "k8s_openapi027_v1_34",
    feature = "k8s_openapi027_v1_35"
))]
#[cfg(test)]
mod test {
    use crate::kube3::ExtractManagedFields;
    use k8s_openapi027::api::apps::v1::{Deployment, DeploymentSpec};
    use k8s_openapi027::api::core::v1::{Container, PodSpec, PodTemplateSpec};
    use k8s_openapi027::apimachinery::pkg::apis::meta::v1::{
        FieldsV1, ManagedFieldsEntry, ObjectMeta,
    };
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::marker::PhantomData;

    #[derive(Serialize, Deserialize, Default, Debug)]
    #[serde(rename_all = "camelCase")]
    struct ApiEnvelopeDeployment {
        #[serde(
            serialize_with = "crate::kube3::serialize_api_version",
            deserialize_with = "crate::kube3::deserialize_api_version"
        )]
        api_version: PhantomData<Deployment>,
        #[serde(
            serialize_with = "crate::kube3::serialize_kind",
            deserialize_with = "crate::kube3::deserialize_kind"
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

    #[test]
    fn extract_deployment() {
        let field_manager = "rust-manager";
        let managed_fields: Value =
            json!({"apiVersion":"apps/v1","kind":"Deployment","f:spec":{"f:replicas": {}}});

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
