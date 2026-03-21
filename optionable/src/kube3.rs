//! Tooling to help deriving optioned types for `kube::CustomResource`.

use crate::Optionable;
use kube3::Resource;
use serde::de::{DeserializeOwned, Error, Unexpected};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};
use std::collections::{BTreeMap, HashMap};
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
    /// The `metadata` entries for `name`, `namespace` and `generateName` are always kept.
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
                && let Value::Object(fields_v) = &fields.0
            {
                let mut data_json = serde_json::to_value(self)?;
                filter_json_value(&mut data_json, fields_v, true, false)?;
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
// Format of managed fields is described here: http://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/#System
#[allow(clippy::too_many_lines)] // just 110 lines and there is not much point in splitting it up
fn filter_json_value(
    item: &mut Value,
    filter: &Map<String, Value>,
    is_root: bool,
    is_metadata: bool,
) -> Result<(), serde_json::Error> {
    if filter.is_empty() && !is_metadata {
        // empty filter means keep all values below this node except for metadata
        return Ok(());
    }
    match item {
        Value::Object(map) => {
            if let Some(unsupported) = filter
                .keys()
                .find(|k| !(k.as_str() == "." || k.starts_with("f:")))
            {
                return Err(serde_json::Error::custom(format!(
                    "Unsupported field manager entry for map entry. This is a bug: `{unsupported}`"
                )));
            }
            let allowed_fields: BTreeMap<_, _> = filter
                .iter()
                .filter_map(|(k, v)| k.strip_prefix("f:").map(|k| (k, v)))
                .collect();

            let mut retain_err = Ok(());
            map.retain(|k, v| {
                if retain_err.is_err() {
                    // short-circuit if we encountered err
                    return false;
                }
                let filter = allowed_fields.get(k.as_str());
                let is_kv_metadata = is_root && k == "metadata";
                if filter.is_none() && !is_kv_metadata {
                    // if we don't have a filter kv_metadata should still be filtered by this logic here in the next recursion step
                    return (is_root && (k == "apiVersion" || k == "kind" || k == "metadata"))
                        || (is_metadata
                            && (k == "name" || k == "generateName" || k == "namespace"));
                }
                retain_err = if let Some(Value::Object(filter_v)) = &filter {
                    filter_json_value(v, filter_v, false, is_kv_metadata)
                } else if is_kv_metadata {
                    filter_json_value(v, &Map::new(), false, is_kv_metadata)
                } else {
                    Ok(())
                };
                true
            });
            retain_err?;
        }
        Value::Array(slice) => {
            if let Some(unsupported) = filter.keys().find(|k| {
                !(k.as_str() == "."
                    || k.starts_with("i:")
                    || k.starts_with("k:")
                    || k.starts_with("v:"))
            }) {
                return Err(serde_json::Error::custom(format!(
                    "Unsupported field manager entry for map entry. This is a bug: `{unsupported}`"
                )));
            }
            let allowed_index = filter
                .iter()
                .filter_map(|(k, v)| k.strip_prefix("i:").map(|k| (k, v)))
                .map(|(k, v)| {
                    Ok((
                        k.parse::<i32>()
                            .map_err(|e| serde_json::Error::custom(e.to_string()))?,
                        v,
                    ))
                })
                .collect::<Result<BTreeMap<_, _>, _>>()?;
            let allowed_keys = filter
                .iter()
                .filter_map(|(k, v)| k.strip_prefix("k:").map(|k| (k, v)))
                .map(|(k, v)| Ok((serde_json::from_str(k)?, v)))
                .collect::<Result<Vec<(Value, _)>, _>>()?;
            let allowed_values = filter
                .iter()
                .filter_map(|(k, v)| k.strip_prefix("v:").map(|k| (k, v)))
                .map(|(k, v)| Ok((serde_json::from_str(k)?, v)))
                .collect::<Result<HashMap<Value, _>, _>>()?;

            let mut retain_err = Ok(());
            let mut index = 0;
            slice.retain_mut(|v| {
                if retain_err.is_err() {
                    // short-circuit if we encountered err
                    return false;
                }
                let mut filter = None;
                // allowed_index
                if let filter_v @ Some(_) = allowed_index.get(&index) {
                    filter = filter_v;
                }
                // allowed_keys
                'outer: for (allowed_key, filter_v) in &allowed_keys {
                    if let Value::Object(allowed_key) = allowed_key {
                        for (allowed_k, allowed_v) in allowed_key {
                            if v.get(allowed_k) != Some(allowed_v) {
                                continue 'outer;
                            }
                        }
                        filter = Some(filter_v);
                    }
                }
                // allowed_values
                if let filter_v @ Some(_) = allowed_values.get(v) {
                    filter = filter_v;
                }
                index += 1;
                if let Some(Value::Object(filter_v)) = filter {
                    retain_err = filter_json_value(v, filter_v, false, false);
                    true
                } else {
                    false
                }
            });
            retain_err?;
        }
        _ => {}
    }
    Ok(())
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
    use crate::k8s_openapi::api::core::v1::ContainerAc;
    use crate::kube3::ExtractManagedFields;
    use k8s_openapi027::api::apps::v1::{Deployment, DeploymentSpec};
    use k8s_openapi027::api::core::v1::{Container, PodSpec, PodTemplateSpec};
    use k8s_openapi027::apimachinery::pkg::apis::meta::v1::{
        FieldsV1, ManagedFieldsEntry, ObjectMeta,
    };
    use serde::{Deserialize, Serialize};
    use serde_json::{Value, json};
    use std::collections::BTreeMap;
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
    fn extract_deployment_err_unsupported_filter() {
        let field_manager = "rust-manager";
        let managed_fields: Value =
            json!({"f:metadata":{"f:labels":{"f:hello2":{}}},"f:spec":{"f:replicas": {}}});

        let deployment = Deployment {
            metadata: ObjectMeta {
                name: Some("my_name".to_owned()),
                namespace: Some("my_namespace".to_owned()),
                labels: Some(BTreeMap::from([
                    ("hello".to_owned(), "world".to_owned()), // not owned, should be removed
                    ("hello2".to_owned(), "world2".to_owned()),
                ])),
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
        let labels = deployment.metadata.labels.as_ref().unwrap();
        let labels_extract = deployment_extract.metadata.labels.as_ref().unwrap();
        assert_eq!(None, labels_extract.get("hello"));
        assert_eq!(
            labels.get("hello2").unwrap(),
            labels_extract.get("hello2").unwrap()
        );
        assert_eq!(
            deployment.spec.unwrap().replicas.unwrap(),
            deployment_extract.spec.as_ref().unwrap().replicas.unwrap()
        );
        assert_eq!(None, deployment_extract.spec.unwrap().template);
    }

    #[test]
    fn extract_deployment() {
        let field_manager = "rust-manager";
        let managed_fields: Value =
            json!({"f:metadata":{"i:0":{"f:hello2":{}}},"f:spec":{"f:replicas": {}}});

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
            ..Default::default()
        };
        let deployment_extract = deployment.clone().extract(field_manager);
        assert!(deployment_extract.is_err());
        assert_eq!(
            "Unsupported field manager entry for map entry. This is a bug: `i:0`",
            deployment_extract.unwrap_err().to_string()
        );
    }

    #[test]
    fn extract_deployment_wrapping_owner() {
        let field_manager = "rust-manager";
        let managed_fields: Value = json!({"f:spec":{}});

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
        assert_ne!(None, deployment_extract.spec.unwrap().template);
    }

    /// Internal helper function to pass in various managed fields that should select the second container
    fn test_extract_deployment_slice_2nd_container(
        managed_fields: Value,
        should_rm_image_pull_policy: bool,
    ) {
        let field_manager = "rust-manager";
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
                        containers: vec![
                            Container {
                                name: "test1".to_owned(),
                                image: Some("test1".to_owned()),
                                image_pull_policy: Some("always".to_owned()), // unowned field, should be removed
                                ..Default::default()
                            },
                            Container {
                                name: "test2".to_owned(),
                                image: Some("test2".to_owned()),
                                image_pull_policy: Some("always".to_owned()), // unowned field, should be removed
                                ..Default::default()
                            },
                            Container {
                                name: "test3".to_owned(),
                                image: Some("test3".to_owned()),
                                image_pull_policy: Some("always".to_owned()), // unowned field, should be removed
                                ..Default::default()
                            },
                        ],
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
        assert_eq!(None, deployment_extract.spec.as_ref().unwrap().replicas);
        let containers = &deployment_extract
            .spec
            .as_ref()
            .unwrap()
            .template
            .as_ref()
            .unwrap()
            .spec
            .as_ref()
            .unwrap()
            .containers
            .as_ref()
            .unwrap();
        assert_eq!(1, containers.iter().len());
        if should_rm_image_pull_policy {
            assert_eq!(
                ContainerAc {
                    name: Some("test2".to_owned()),
                    image: Some("test2".to_owned()),
                    ..Default::default()
                },
                containers[0]
            );
        } else {
            assert_eq!(
                ContainerAc {
                    name: Some("test2".to_owned()),
                    image: Some("test2".to_owned()),
                    image_pull_policy: Some("always".to_owned()),
                    ..Default::default()
                },
                containers[0]
            );
        }
    }

    #[test]
    fn extract_deployment_slice_index() {
        test_extract_deployment_slice_2nd_container(
            json!({"f:spec":{"f:template":{"f:spec":{"f:containers":{"i:1":{"f:name":{},"f:image":{}}}}}}}),
            true,
        );
    }

    #[test]
    fn extract_deployment_slice_key() {
        test_extract_deployment_slice_2nd_container(
            json!({"f:spec":{"f:template":{"f:spec":{"f:containers":{"k:{\"name\":\"test2\"}":{"f:name":{},"f:image":{}}}}}}}),
            true,
        );
    }

    #[test]
    fn extract_deployment_value() {
        test_extract_deployment_slice_2nd_container(
            json!({"f:spec":{"f:template":{"f:spec":{"f:containers":{"v:{\"name\":\"test2\",\"image\":\"test2\",\"imagePullPolicy\":\"always\"}":{}}}}}}),
            false,
        );
    }
}
