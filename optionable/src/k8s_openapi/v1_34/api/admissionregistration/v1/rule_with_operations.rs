pub struct RuleWithOperationsAc {
    pub api_groups: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub api_versions: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub operations: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub resources: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub scope: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1::RuleWithOperations {
    type Optioned = RuleWithOperationsAc;
}
#[automatically_derived]
impl crate::Optionable for RuleWithOperationsAc {
    type Optioned = RuleWithOperationsAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::RuleWithOperations {
    fn into_optioned(self) -> RuleWithOperationsAc {
        RuleWithOperationsAc {
            api_groups: crate::OptionableConvert::into_optioned(self.api_groups),
            api_versions: crate::OptionableConvert::into_optioned(self.api_versions),
            operations: crate::OptionableConvert::into_optioned(self.operations),
            resources: crate::OptionableConvert::into_optioned(self.resources),
            scope: crate::OptionableConvert::into_optioned(self.scope),
        }
    }
    fn try_from_optioned(
        value: RuleWithOperationsAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_groups: crate::OptionableConvert::try_from_optioned(value.api_groups)?,
            api_versions: crate::OptionableConvert::try_from_optioned(
                value.api_versions,
            )?,
            operations: crate::OptionableConvert::try_from_optioned(value.operations)?,
            resources: crate::OptionableConvert::try_from_optioned(value.resources)?,
            scope: crate::OptionableConvert::try_from_optioned(value.scope)?,
        })
    }
    fn merge(
        &mut self,
        other: RuleWithOperationsAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.api_groups, other.api_groups)?;
        crate::OptionableConvert::merge(&mut self.api_versions, other.api_versions)?;
        crate::OptionableConvert::merge(&mut self.operations, other.operations)?;
        crate::OptionableConvert::merge(&mut self.resources, other.resources)?;
        crate::OptionableConvert::merge(&mut self.scope, other.scope)?;
        Ok(())
    }
}
