pub struct CustomResourceDefinitionStatusOpt {
    pub accepted_names: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames,
    > as crate::Optionable>::Optioned,
    pub conditions: <Option<
        std::vec::Vec<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionCondition,
        >,
    > as crate::Optionable>::Optioned,
    pub stored_versions: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionStatus {
    type Optioned = CustomResourceDefinitionStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceDefinitionStatusOpt {
    type Optioned = CustomResourceDefinitionStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionStatus {
    fn into_optioned(self) -> CustomResourceDefinitionStatusOpt {
        CustomResourceDefinitionStatusOpt {
            accepted_names: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames,
            > as crate::OptionableConvert>::into_optioned(self.accepted_names),
            conditions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionCondition,
                >,
            > as crate::OptionableConvert>::into_optioned(self.conditions),
            stored_versions: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.stored_versions),
        }
    }
    fn try_from_optioned(
        value: CustomResourceDefinitionStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            accepted_names: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames,
            > as crate::OptionableConvert>::try_from_optioned(value.accepted_names)?,
            conditions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionCondition,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.conditions)?,
            stored_versions: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.stored_versions)?,
        })
    }
    fn merge(
        &mut self,
        other: CustomResourceDefinitionStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames,
        > as crate::OptionableConvert>::merge(
            &mut self.accepted_names,
            other.accepted_names,
        )?;
        <Option<
            std::vec::Vec<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionCondition,
            >,
        > as crate::OptionableConvert>::merge(&mut self.conditions, other.conditions)?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.stored_versions,
            other.stored_versions,
        )?;
        Ok(())
    }
}
