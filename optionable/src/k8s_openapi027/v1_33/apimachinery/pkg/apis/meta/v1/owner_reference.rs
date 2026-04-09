#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// OwnerReference contains enough information to let you identify an owning object. An owning object must be in the same namespace as the dependent, or be cluster-scoped, so there is no namespace field.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OwnerReferenceAc {
    /// API version of the referent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<std::string::String>,
    /// If true, AND if the owner has the "foregroundDeletion" finalizer, then the owner cannot be deleted from the key-value store until this reference is removed. See https://kubernetes.io/docs/concepts/architecture/garbage-collection/#foreground-deletion for how the garbage collector interacts with this field and enforces the foreground deletion. Defaults to false. To set this field, a user needs "delete" permission of the owner, otherwise 422 (Unprocessable Entity) will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_owner_deletion: Option<bool>,
    /// If true, this reference points to the managing controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<bool>,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<std::string::String>,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names#names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names#uids
    pub uid: std::string::String,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::OwnerReference {
    type Optioned = OwnerReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for OwnerReferenceAc {
    type Optioned = OwnerReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::OwnerReference {
    fn into_optioned(self) -> OwnerReferenceAc {
        OwnerReferenceAc {
            api_version: Some(self.api_version),
            block_owner_deletion: self.block_owner_deletion,
            controller: self.controller,
            kind: Some(self.kind),
            name: Some(self.name),
            uid: self.uid,
        }
    }
    fn try_from_optioned(value: OwnerReferenceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_version: value
                .api_version
                .ok_or(crate::Error {
                    missing_field: "api_version",
                })?,
            block_owner_deletion: value.block_owner_deletion,
            controller: value.controller,
            kind: value
                .kind
                .ok_or(crate::Error {
                    missing_field: "kind",
                })?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            uid: value.uid,
        })
    }
    fn merge(&mut self, other: OwnerReferenceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.api_version {
            self.api_version = other_value;
        }
        self.block_owner_deletion = other.block_owner_deletion;
        self.controller = other.controller;
        if let Some(other_value) = other.kind {
            self.kind = other_value;
        }
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        self.uid = other.uid;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apimachinery::pkg::apis::meta::v1::OwnerReference,
> for OwnerReferenceAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::OwnerReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::OwnerReference,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::OwnerReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
