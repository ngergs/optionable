#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OwnerReferenceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_owner_deletion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<std::string::String>,
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
            uid: Some(self.uid),
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
            uid: value
                .uid
                .ok_or(crate::Error {
                    missing_field: "uid",
                })?,
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
        if let Some(other_value) = other.uid {
            self.uid = other_value;
        }
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
