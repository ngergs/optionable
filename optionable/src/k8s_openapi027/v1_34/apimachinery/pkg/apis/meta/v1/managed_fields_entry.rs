#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ManagedFieldsEntryAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_type: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_v1: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::FieldsV1 as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subresource: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry {
    type Optioned = ManagedFieldsEntryAc;
}
#[automatically_derived]
impl crate::Optionable for ManagedFieldsEntryAc {
    type Optioned = ManagedFieldsEntryAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry {
    fn into_optioned(self) -> ManagedFieldsEntryAc {
        ManagedFieldsEntryAc {
            api_version: self.api_version,
            fields_type: self.fields_type,
            fields_v1: crate::OptionableConvert::into_optioned(self.fields_v1),
            manager: self.manager,
            operation: self.operation,
            subresource: self.subresource,
            time: crate::OptionableConvert::into_optioned(self.time),
        }
    }
    fn try_from_optioned(value: ManagedFieldsEntryAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_version: value.api_version,
            fields_type: value.fields_type,
            fields_v1: crate::OptionableConvert::try_from_optioned(value.fields_v1)?,
            manager: value.manager,
            operation: value.operation,
            subresource: value.subresource,
            time: crate::OptionableConvert::try_from_optioned(value.time)?,
        })
    }
    fn merge(&mut self, other: ManagedFieldsEntryAc) -> Result<(), crate::Error> {
        self.api_version = other.api_version;
        self.fields_type = other.fields_type;
        crate::OptionableConvert::merge(&mut self.fields_v1, other.fields_v1)?;
        self.manager = other.manager;
        self.operation = other.operation;
        self.subresource = other.subresource;
        crate::OptionableConvert::merge(&mut self.time, other.time)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry,
> for ManagedFieldsEntryAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
