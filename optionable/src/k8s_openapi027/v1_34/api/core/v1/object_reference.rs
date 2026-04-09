#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ObjectReference contains enough information to let you inspect or modify the referred object.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ObjectReferenceAc {
    /// API version of the referent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<std::string::String>,
    /// If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers\[2\]. For example, if the object reference is to a container within a pod, this would take on a value like: "spec.containers{name}" (where "name" refers to the name of the container that triggered the event) or if no container name is specified "spec.containers\[2\]" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_path: Option<std::string::String>,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<std::string::String>,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: Option<std::string::String>,
    /// Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<std::string::String>,
    /// Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<std::string::String>,
    /// UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ObjectReference {
    type Optioned = ObjectReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for ObjectReferenceAc {
    type Optioned = ObjectReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ObjectReference {
    fn into_optioned(self) -> ObjectReferenceAc {
        ObjectReferenceAc {
            api_version: self.api_version,
            field_path: self.field_path,
            kind: self.kind,
            name: self.name,
            namespace: self.namespace,
            resource_version: self.resource_version,
            uid: self.uid,
        }
    }
    fn try_from_optioned(value: ObjectReferenceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_version: value.api_version,
            field_path: value.field_path,
            kind: value.kind,
            name: value.name,
            namespace: value.namespace,
            resource_version: value.resource_version,
            uid: value.uid,
        })
    }
    fn merge(&mut self, other: ObjectReferenceAc) -> Result<(), crate::Error> {
        self.api_version = other.api_version;
        self.field_path = other.field_path;
        self.kind = other.kind;
        self.name = other.name;
        self.namespace = other.namespace;
        self.resource_version = other.resource_version;
        self.uid = other.uid;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ObjectReference>
for ObjectReferenceAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::ObjectReference) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ObjectReference, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ObjectReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
