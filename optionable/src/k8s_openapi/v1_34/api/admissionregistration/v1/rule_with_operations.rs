pub struct RuleWithOperationsOpt {
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
for ::k8s_openapi::api::admissionregistration::v1::rule_with_operations::RuleWithOperations {
    type Optioned = RuleWithOperationsOpt;
}
#[automatically_derived]
impl crate::Optionable for RuleWithOperationsOpt {
    type Optioned = RuleWithOperationsOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::rule_with_operations::RuleWithOperations {
    fn into_optioned(self) -> RuleWithOperationsOpt {
        RuleWithOperationsOpt {
            api_groups: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.api_groups),
            api_versions: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.api_versions),
            operations: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.operations),
            resources: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.resources),
            scope: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.scope),
        }
    }
    fn try_from_optioned(
        value: RuleWithOperationsOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_groups: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.api_groups)?,
            api_versions: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.api_versions)?,
            operations: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.operations)?,
            resources: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.resources)?,
            scope: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.scope)?,
        })
    }
    fn merge(
        &mut self,
        other: RuleWithOperationsOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.api_groups, other.api_groups)?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.api_versions,
            other.api_versions,
        )?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.operations, other.operations)?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.resources, other.resources)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.scope, other.scope)?;
        Ok(())
    }
}
