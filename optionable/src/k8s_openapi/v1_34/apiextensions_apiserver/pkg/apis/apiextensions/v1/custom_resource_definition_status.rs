#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceDefinitionStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_names: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionCondition,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_versions: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionStatus {
    type Optioned = CustomResourceDefinitionStatusAc;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceDefinitionStatusAc {
    type Optioned = CustomResourceDefinitionStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionStatus {
    fn into_optioned(self) -> CustomResourceDefinitionStatusAc {
        CustomResourceDefinitionStatusAc {
            accepted_names: crate::OptionableConvert::into_optioned(self.accepted_names),
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            stored_versions: crate::OptionableConvert::into_optioned(
                self.stored_versions,
            ),
        }
    }
    fn try_from_optioned(
        value: CustomResourceDefinitionStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            accepted_names: crate::OptionableConvert::try_from_optioned(
                value.accepted_names,
            )?,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            stored_versions: crate::OptionableConvert::try_from_optioned(
                value.stored_versions,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: CustomResourceDefinitionStatusAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.accepted_names, other.accepted_names)?;
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        crate::OptionableConvert::merge(
            &mut self.stored_versions,
            other.stored_versions,
        )?;
        Ok(())
    }
}
