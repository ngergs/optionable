pub struct ManagedFieldsEntryOpt {
    pub api_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub fields_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub fields_v1: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::FieldsV1,
    > as crate::Optionable>::Optioned,
    pub manager: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub operation: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub subresource: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry {
    type Optioned = ManagedFieldsEntryOpt;
}
#[automatically_derived]
impl crate::Optionable for ManagedFieldsEntryOpt {
    type Optioned = ManagedFieldsEntryOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry {
    fn into_optioned(self) -> ManagedFieldsEntryOpt {
        ManagedFieldsEntryOpt {
            api_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.api_version),
            fields_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.fields_type),
            fields_v1: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::FieldsV1,
            > as crate::OptionableConvert>::into_optioned(self.fields_v1),
            manager: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.manager),
            operation: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.operation),
            subresource: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.subresource),
            time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::into_optioned(self.time),
        }
    }
    fn try_from_optioned(
        value: ManagedFieldsEntryOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.api_version)?,
            fields_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.fields_type)?,
            fields_v1: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::FieldsV1,
            > as crate::OptionableConvert>::try_from_optioned(value.fields_v1)?,
            manager: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.manager)?,
            operation: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.operation)?,
            subresource: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.subresource)?,
            time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::try_from_optioned(value.time)?,
        })
    }
    fn merge(
        &mut self,
        other: ManagedFieldsEntryOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.api_version, other.api_version)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.fields_type, other.fields_type)?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::FieldsV1,
        > as crate::OptionableConvert>::merge(&mut self.fields_v1, other.fields_v1)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.manager, other.manager)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.operation, other.operation)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.subresource, other.subresource)?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
        > as crate::OptionableConvert>::merge(&mut self.time, other.time)?;
        Ok(())
    }
}
