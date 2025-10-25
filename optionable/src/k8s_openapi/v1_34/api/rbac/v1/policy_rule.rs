pub struct PolicyRuleOpt {
    pub api_groups: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub non_resource_urls: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub resource_names: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub resources: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub verbs: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::rbac::v1::policy_rule::PolicyRule {
    type Optioned = PolicyRuleOpt;
}
#[automatically_derived]
impl crate::Optionable for PolicyRuleOpt {
    type Optioned = PolicyRuleOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::rbac::v1::policy_rule::PolicyRule {
    fn into_optioned(self) -> PolicyRuleOpt {
        PolicyRuleOpt {
            api_groups: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.api_groups),
            non_resource_urls: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.non_resource_urls),
            resource_names: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.resource_names),
            resources: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.resources),
            verbs: Some(
                <std::vec::Vec<
                    std::string::String,
                > as crate::OptionableConvert>::into_optioned(self.verbs),
            ),
        }
    }
    fn try_from_optioned(
        value: PolicyRuleOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_groups: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.api_groups)?,
            non_resource_urls: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.non_resource_urls)?,
            resource_names: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.resource_names)?,
            resources: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.resources)?,
            verbs: <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .verbs
                    .ok_or(crate::optionable::Error {
                        missing_field: "verbs",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: PolicyRuleOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.api_groups, other.api_groups)?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.non_resource_urls,
            other.non_resource_urls,
        )?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.resource_names,
            other.resource_names,
        )?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.resources, other.resources)?;
        if let Some(other_value) = other.verbs {
            <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::merge(&mut self.verbs, other_value)?;
        }
        Ok(())
    }
}
