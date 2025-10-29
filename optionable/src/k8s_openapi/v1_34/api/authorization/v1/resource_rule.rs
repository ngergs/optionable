pub struct ResourceRuleOpt {
    pub api_groups: <Option<
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
impl crate::Optionable for ::k8s_openapi::api::authorization::v1::ResourceRule {
    type Optioned = ResourceRuleOpt;
}
#[automatically_derived]
impl crate::Optionable for ResourceRuleOpt {
    type Optioned = ResourceRuleOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::authorization::v1::ResourceRule {
    fn into_optioned(self) -> ResourceRuleOpt {
        ResourceRuleOpt {
            api_groups: crate::OptionableConvert::into_optioned(self.api_groups),
            resource_names: crate::OptionableConvert::into_optioned(self.resource_names),
            resources: crate::OptionableConvert::into_optioned(self.resources),
            verbs: Some(crate::OptionableConvert::into_optioned(self.verbs)),
        }
    }
    fn try_from_optioned(
        value: ResourceRuleOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_groups: crate::OptionableConvert::try_from_optioned(value.api_groups)?,
            resource_names: crate::OptionableConvert::try_from_optioned(
                value.resource_names,
            )?,
            resources: crate::OptionableConvert::try_from_optioned(value.resources)?,
            verbs: crate::OptionableConvert::try_from_optioned(
                value
                    .verbs
                    .ok_or(crate::optionable::Error {
                        missing_field: "verbs",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: ResourceRuleOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.api_groups, other.api_groups)?;
        crate::OptionableConvert::merge(&mut self.resource_names, other.resource_names)?;
        crate::OptionableConvert::merge(&mut self.resources, other.resources)?;
        if let Some(other_value) = other.verbs {
            crate::OptionableConvert::merge(&mut self.verbs, other_value)?;
        }
        Ok(())
    }
}
