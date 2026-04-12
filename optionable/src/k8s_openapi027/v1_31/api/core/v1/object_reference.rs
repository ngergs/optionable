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
        if self.api_version.is_none() {
            self.api_version = crate::OptionableConvert::try_from_optioned(
                other.api_version,
            )?;
        } else if let Some(self_value) = self.api_version.as_mut()
            && let Some(other_value) = other.api_version
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.field_path.is_none() {
            self.field_path = crate::OptionableConvert::try_from_optioned(
                other.field_path,
            )?;
        } else if let Some(self_value) = self.field_path.as_mut()
            && let Some(other_value) = other.field_path
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.kind.is_none() {
            self.kind = crate::OptionableConvert::try_from_optioned(other.kind)?;
        } else if let Some(self_value) = self.kind.as_mut()
            && let Some(other_value) = other.kind
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        self.name = other.name;
        if self.namespace.is_none() {
            self.namespace = crate::OptionableConvert::try_from_optioned(
                other.namespace,
            )?;
        } else if let Some(self_value) = self.namespace.as_mut()
            && let Some(other_value) = other.namespace
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.resource_version.is_none() {
            self.resource_version = crate::OptionableConvert::try_from_optioned(
                other.resource_version,
            )?;
        } else if let Some(self_value) = self.resource_version.as_mut()
            && let Some(other_value) = other.resource_version
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.uid.is_none() {
            self.uid = crate::OptionableConvert::try_from_optioned(other.uid)?;
        } else if let Some(self_value) = self.uid.as_mut()
            && let Some(other_value) = other.uid
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi027::api::core::v1::ObjectReference {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.name == other.name
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
