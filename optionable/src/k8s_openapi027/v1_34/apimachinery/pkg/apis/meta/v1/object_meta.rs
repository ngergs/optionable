#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ObjectMeta is metadata that all persisted resources must have, which includes all objects users must create.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ObjectMetaAc {
    /// Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
    /// CreationTimestamp is a timestamp representing the server time when this object was created. It is not guaranteed to be set in happens-before order across separate operations. Clients may not set this value. It is represented in RFC3339 form and is in UTC.
    ///
    /// Populated by the system. Read-only. Null for lists. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// Number of seconds allowed for this object to gracefully terminate before it will be removed from the system. Only set when deletionTimestamp is also set. May only be shortened. Read-only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_grace_period_seconds: Option<i64>,
    /// DeletionTimestamp is RFC 3339 date and time at which this resource will be deleted. This field is set by the server when a graceful deletion is requested by the user, and is not directly settable by a client. The resource is expected to be deleted (no longer visible from resource lists, and not reachable by name) after the time in this field, once the finalizers list is empty. As long as the finalizers list contains items, deletion is blocked. Once the deletionTimestamp is set, this value may not be unset or be set further into the future, although it may be shortened or the resource may be deleted prior to this time. For example, a user may request that a pod is deleted in 30 seconds. The Kubelet will react by sending a graceful termination signal to the containers in the pod. After that 30 seconds, the Kubelet will send a hard termination signal (SIGKILL) to the container and after cleanup, remove the pod from the API. In the presence of network partitions, this object may still exist after this timestamp, until an administrator or automated process can determine the resource is fully terminated. If not set, graceful deletion of the object has not been requested.
    ///
    /// Populated by the system when a graceful deletion is requested. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_timestamp: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// Must be empty before the object is deleted from the registry. Each entry is an identifier for the responsible component that will remove the entry from the list. If the deletionTimestamp of the object is non-nil, entries in this list can only be removed. Finalizers may be processed and removed in any order.  Order is NOT enforced because it introduces significant risk of stuck finalizers. finalizers is a shared field, any actor with permission can reorder it. If the finalizer list is processed in order, then this can lead to a situation in which the component responsible for the first finalizer in the list is waiting for a signal (field value, external system, or other) produced by a component responsible for a finalizer later in the list, resulting in a deadlock. Without enforced ordering finalizers are free to order amongst themselves and are not vulnerable to ordering changes in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalizers: Option<std::vec::Vec<std::string::String>>,
    /// GenerateName is an optional prefix, used by the server, to generate a unique name ONLY IF the Name field has not been provided. If this field is used, the name returned to the client will be different than the name passed. This value will also be combined with a unique suffix. The provided value has the same validation rules as the Name field, and may be truncated by the length of the suffix required to make the value unique on the server.
    ///
    /// If this field is specified and the generated name exists, the server will return a 409.
    ///
    /// Applied only if Name is not specified. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#idempotency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_name: Option<std::string::String>,
    /// A sequence number representing a specific generation of the desired state. Populated by the system. Read-only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation: Option<i64>,
    /// Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and services. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
    /// ManagedFields maps workflow-id and version to the set of fields that are managed by that workflow. This is mostly for internal housekeeping, and users typically shouldn't need to set or understand this field. A workflow can be the user's name, a controller's name, or the name of a specific apply path like "ci-cd". The set of fields is always in the version that the workflow used when modifying the object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_fields: Option<
        std::vec::Vec<
            <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry as crate::Optionable>::Optioned,
        >,
    >,
    /// Name must be unique within a namespace. Is required when creating resources, although some resources may allow a client to request the generation of an appropriate name automatically. Name is primarily intended for creation idempotence and configuration definition. Cannot be updated. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names#names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// Namespace defines the space within which each name must be unique. An empty namespace is equivalent to the "default" namespace, but "default" is the canonical representation. Not all objects are required to be scoped to a namespace - the value of this field for those objects will be empty.
    ///
    /// Must be a DNS_LABEL. Cannot be updated. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<std::string::String>,
    /// List of objects depended by this object. If ALL objects in the list have been deleted, this object will be garbage collected. If this object is managed by a controller, then an entry in this list will point to this controller, with the controller field set to true. There cannot be more than one managing controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_references: Option<
        std::vec::Vec<
            <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::OwnerReference as crate::Optionable>::Optioned,
        >,
    >,
    /// An opaque value that represents the internal version of this object that can be used by clients to determine when objects have changed. May be used for optimistic concurrency, change detection, and the watch operation on a resource or set of resources. Clients must treat these values as opaque and passed unmodified back to the server. They may only be valid for a particular resource or set of resources.
    ///
    /// Populated by the system. Read-only. Value must be treated as opaque by clients and . More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<std::string::String>,
    /// Deprecated: selfLink is a legacy read-only field that is no longer populated by the system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<std::string::String>,
    /// UID is the unique in time and space value for this object. It is typically generated by the server on successful creation of a resource and is not allowed to change on PUT operations.
    ///
    /// Populated by the system. Read-only. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names#uids
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta {
    type Optioned = ObjectMetaAc;
}
#[automatically_derived]
impl crate::Optionable for ObjectMetaAc {
    type Optioned = ObjectMetaAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta {
    fn into_optioned(self) -> ObjectMetaAc {
        ObjectMetaAc {
            annotations: self.annotations,
            creation_timestamp: crate::OptionableConvert::into_optioned(
                self.creation_timestamp,
            ),
            deletion_grace_period_seconds: self.deletion_grace_period_seconds,
            deletion_timestamp: crate::OptionableConvert::into_optioned(
                self.deletion_timestamp,
            ),
            finalizers: self.finalizers,
            generate_name: self.generate_name,
            generation: self.generation,
            labels: self.labels,
            managed_fields: crate::OptionableConvert::into_optioned(self.managed_fields),
            name: self.name,
            namespace: self.namespace,
            owner_references: crate::OptionableConvert::into_optioned(
                self.owner_references,
            ),
            resource_version: self.resource_version,
            self_link: self.self_link,
            uid: self.uid,
        }
    }
    fn try_from_optioned(value: ObjectMetaAc) -> Result<Self, crate::Error> {
        Ok(Self {
            annotations: value.annotations,
            creation_timestamp: crate::OptionableConvert::try_from_optioned(
                value.creation_timestamp,
            )?,
            deletion_grace_period_seconds: value.deletion_grace_period_seconds,
            deletion_timestamp: crate::OptionableConvert::try_from_optioned(
                value.deletion_timestamp,
            )?,
            finalizers: value.finalizers,
            generate_name: value.generate_name,
            generation: value.generation,
            labels: value.labels,
            managed_fields: crate::OptionableConvert::try_from_optioned(
                value.managed_fields,
            )?,
            name: value.name,
            namespace: value.namespace,
            owner_references: crate::OptionableConvert::try_from_optioned(
                value.owner_references,
            )?,
            resource_version: value.resource_version,
            self_link: value.self_link,
            uid: value.uid,
        })
    }
    fn merge(&mut self, other: ObjectMetaAc) -> Result<(), crate::Error> {
        if self.annotations.is_none() {
            self.annotations = crate::OptionableConvert::try_from_optioned(
                other.annotations,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.annotations, other.annotations)?;
        }
        if self.creation_timestamp.is_none() {
            self.creation_timestamp = crate::OptionableConvert::try_from_optioned(
                other.creation_timestamp,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.creation_timestamp,
                other.creation_timestamp,
            )?;
        }
        if self.deletion_grace_period_seconds.is_none() {
            self.deletion_grace_period_seconds = crate::OptionableConvert::try_from_optioned(
                other.deletion_grace_period_seconds,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.deletion_grace_period_seconds,
                other.deletion_grace_period_seconds,
            )?;
        }
        if self.deletion_timestamp.is_none() {
            self.deletion_timestamp = crate::OptionableConvert::try_from_optioned(
                other.deletion_timestamp,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.deletion_timestamp,
                other.deletion_timestamp,
            )?;
        }
        if self.finalizers.is_none() {
            self.finalizers = crate::OptionableConvert::try_from_optioned(
                other.finalizers,
            )?;
        } else {
            crate::merge::try_merge_optioned_set(
                &mut self.finalizers,
                other.finalizers,
            )?;
        }
        if self.generate_name.is_none() {
            self.generate_name = crate::OptionableConvert::try_from_optioned(
                other.generate_name,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.generate_name,
                other.generate_name,
            )?;
        }
        if self.generation.is_none() {
            self.generation = crate::OptionableConvert::try_from_optioned(
                other.generation,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.generation, other.generation)?;
        }
        if self.labels.is_none() {
            self.labels = crate::OptionableConvert::try_from_optioned(other.labels)?;
        } else {
            crate::OptionableConvert::merge(&mut self.labels, other.labels)?;
        }
        if self.managed_fields.is_none() {
            self.managed_fields = crate::OptionableConvert::try_from_optioned(
                other.managed_fields,
            )?;
        } else {
            self.managed_fields = crate::OptionableConvert::try_from_optioned(
                other.managed_fields,
            )?;
        }
        if self.name.is_none() {
            self.name = crate::OptionableConvert::try_from_optioned(other.name)?;
        } else {
            crate::OptionableConvert::merge(&mut self.name, other.name)?;
        }
        if self.namespace.is_none() {
            self.namespace = crate::OptionableConvert::try_from_optioned(
                other.namespace,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.namespace, other.namespace)?;
        }
        if self.owner_references.is_none() {
            self.owner_references = crate::OptionableConvert::try_from_optioned(
                other.owner_references,
            )?;
        } else {
            crate::merge::try_merge_optioned_map(
                &mut self.owner_references,
                other.owner_references,
            )?;
        }
        if self.resource_version.is_none() {
            self.resource_version = crate::OptionableConvert::try_from_optioned(
                other.resource_version,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.resource_version,
                other.resource_version,
            )?;
        }
        if self.self_link.is_none() {
            self.self_link = crate::OptionableConvert::try_from_optioned(
                other.self_link,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.self_link, other.self_link)?;
        }
        if self.uid.is_none() {
            self.uid = crate::OptionableConvert::try_from_optioned(other.uid)?;
        } else {
            crate::OptionableConvert::merge(&mut self.uid, other.uid)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
> for ObjectMetaAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
