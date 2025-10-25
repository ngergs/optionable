pub struct CustomResourceDefinitionSpecOpt {
    pub conversion: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion,
    > as crate::Optionable>::Optioned,
    pub group: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub names: Option<
        <::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames as crate::Optionable>::Optioned,
    >,
    pub preserve_unknown_fields: <Option<bool> as crate::Optionable>::Optioned,
    pub scope: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub versions: Option<
        <std::vec::Vec<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionVersion,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionSpec {
    type Optioned = CustomResourceDefinitionSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceDefinitionSpecOpt {
    type Optioned = CustomResourceDefinitionSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionSpec {
    fn into_optioned(self) -> CustomResourceDefinitionSpecOpt {
        CustomResourceDefinitionSpecOpt {
            conversion: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion,
            > as crate::OptionableConvert>::into_optioned(self.conversion),
            group: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.group,
                ),
            ),
            names: Some(
                <::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames as crate::OptionableConvert>::into_optioned(
                    self.names,
                ),
            ),
            preserve_unknown_fields: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.preserve_unknown_fields),
            scope: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.scope,
                ),
            ),
            versions: Some(
                <std::vec::Vec<
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionVersion,
                > as crate::OptionableConvert>::into_optioned(self.versions),
            ),
        }
    }
    fn try_from_optioned(
        value: CustomResourceDefinitionSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conversion: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion,
            > as crate::OptionableConvert>::try_from_optioned(value.conversion)?,
            group: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .group
                    .ok_or(crate::optionable::Error {
                        missing_field: "group",
                    })?,
            )?,
            names: <::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames as crate::OptionableConvert>::try_from_optioned(
                value
                    .names
                    .ok_or(crate::optionable::Error {
                        missing_field: "names",
                    })?,
            )?,
            preserve_unknown_fields: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(
                value.preserve_unknown_fields,
            )?,
            scope: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .scope
                    .ok_or(crate::optionable::Error {
                        missing_field: "scope",
                    })?,
            )?,
            versions: <std::vec::Vec<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionVersion,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .versions
                    .ok_or(crate::optionable::Error {
                        missing_field: "versions",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: CustomResourceDefinitionSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion,
        > as crate::OptionableConvert>::merge(&mut self.conversion, other.conversion)?;
        if let Some(other_value) = other.group {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.group,
                other_value,
            )?;
        }
        if let Some(other_value) = other.names {
            <::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames as crate::OptionableConvert>::merge(
                &mut self.names,
                other_value,
            )?;
        }
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.preserve_unknown_fields,
            other.preserve_unknown_fields,
        )?;
        if let Some(other_value) = other.scope {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.scope,
                other_value,
            )?;
        }
        if let Some(other_value) = other.versions {
            <std::vec::Vec<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionVersion,
            > as crate::OptionableConvert>::merge(&mut self.versions, other_value)?;
        }
        Ok(())
    }
}
