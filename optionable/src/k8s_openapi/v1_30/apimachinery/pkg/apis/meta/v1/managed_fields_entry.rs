#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct ManagedFieldsEntryAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_v1: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::FieldsV1,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subresource: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry {
    type Optioned = ManagedFieldsEntryAc;
}
#[automatically_derived]
impl crate::Optionable for ManagedFieldsEntryAc {
    type Optioned = ManagedFieldsEntryAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry {
    fn into_optioned(self) -> ManagedFieldsEntryAc {
        ManagedFieldsEntryAc {
            api_version: crate::OptionableConvert::into_optioned(self.api_version),
            fields_type: crate::OptionableConvert::into_optioned(self.fields_type),
            fields_v1: crate::OptionableConvert::into_optioned(self.fields_v1),
            manager: crate::OptionableConvert::into_optioned(self.manager),
            operation: crate::OptionableConvert::into_optioned(self.operation),
            subresource: crate::OptionableConvert::into_optioned(self.subresource),
            time: crate::OptionableConvert::into_optioned(self.time),
        }
    }
    fn try_from_optioned(value: ManagedFieldsEntryAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_version: crate::OptionableConvert::try_from_optioned(value.api_version)?,
            fields_type: crate::OptionableConvert::try_from_optioned(value.fields_type)?,
            fields_v1: crate::OptionableConvert::try_from_optioned(value.fields_v1)?,
            manager: crate::OptionableConvert::try_from_optioned(value.manager)?,
            operation: crate::OptionableConvert::try_from_optioned(value.operation)?,
            subresource: crate::OptionableConvert::try_from_optioned(value.subresource)?,
            time: crate::OptionableConvert::try_from_optioned(value.time)?,
        })
    }
    fn merge(&mut self, other: ManagedFieldsEntryAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.api_version, other.api_version)?;
        crate::OptionableConvert::merge(&mut self.fields_type, other.fields_type)?;
        crate::OptionableConvert::merge(&mut self.fields_v1, other.fields_v1)?;
        crate::OptionableConvert::merge(&mut self.manager, other.manager)?;
        crate::OptionableConvert::merge(&mut self.operation, other.operation)?;
        crate::OptionableConvert::merge(&mut self.subresource, other.subresource)?;
        crate::OptionableConvert::merge(&mut self.time, other.time)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry,
> for ManagedFieldsEntryAc {
    fn from_optionable(
        value: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
