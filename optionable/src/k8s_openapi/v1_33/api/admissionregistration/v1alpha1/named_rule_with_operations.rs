#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct NamedRuleWithOperationsAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_groups: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_versions: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_names: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1alpha1::NamedRuleWithOperations {
    type Optioned = NamedRuleWithOperationsAc;
}
#[automatically_derived]
impl crate::Optionable for NamedRuleWithOperationsAc {
    type Optioned = NamedRuleWithOperationsAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1alpha1::NamedRuleWithOperations {
    fn into_optioned(self) -> NamedRuleWithOperationsAc {
        NamedRuleWithOperationsAc {
            api_groups: crate::OptionableConvert::into_optioned(self.api_groups),
            api_versions: crate::OptionableConvert::into_optioned(self.api_versions),
            operations: crate::OptionableConvert::into_optioned(self.operations),
            resource_names: crate::OptionableConvert::into_optioned(self.resource_names),
            resources: crate::OptionableConvert::into_optioned(self.resources),
            scope: crate::OptionableConvert::into_optioned(self.scope),
        }
    }
    fn try_from_optioned(
        value: NamedRuleWithOperationsAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            api_groups: crate::OptionableConvert::try_from_optioned(value.api_groups)?,
            api_versions: crate::OptionableConvert::try_from_optioned(
                value.api_versions,
            )?,
            operations: crate::OptionableConvert::try_from_optioned(value.operations)?,
            resource_names: crate::OptionableConvert::try_from_optioned(
                value.resource_names,
            )?,
            resources: crate::OptionableConvert::try_from_optioned(value.resources)?,
            scope: crate::OptionableConvert::try_from_optioned(value.scope)?,
        })
    }
    fn merge(&mut self, other: NamedRuleWithOperationsAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.api_groups, other.api_groups)?;
        crate::OptionableConvert::merge(&mut self.api_versions, other.api_versions)?;
        crate::OptionableConvert::merge(&mut self.operations, other.operations)?;
        crate::OptionableConvert::merge(&mut self.resource_names, other.resource_names)?;
        crate::OptionableConvert::merge(&mut self.resources, other.resources)?;
        crate::OptionableConvert::merge(&mut self.scope, other.scope)?;
        Ok(())
    }
}
