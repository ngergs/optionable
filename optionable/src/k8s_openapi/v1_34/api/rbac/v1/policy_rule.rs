#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyRuleAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_groups: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_resource_urls: <Option<
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
    pub verbs: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::rbac::v1::PolicyRule {
    type Optioned = PolicyRuleAc;
}
#[automatically_derived]
impl crate::Optionable for PolicyRuleAc {
    type Optioned = PolicyRuleAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::rbac::v1::PolicyRule {
    fn into_optioned(self) -> PolicyRuleAc {
        PolicyRuleAc {
            api_groups: crate::OptionableConvert::into_optioned(self.api_groups),
            non_resource_urls: crate::OptionableConvert::into_optioned(
                self.non_resource_urls,
            ),
            resource_names: crate::OptionableConvert::into_optioned(self.resource_names),
            resources: crate::OptionableConvert::into_optioned(self.resources),
            verbs: Some(crate::OptionableConvert::into_optioned(self.verbs)),
        }
    }
    fn try_from_optioned(value: PolicyRuleAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_groups: crate::OptionableConvert::try_from_optioned(value.api_groups)?,
            non_resource_urls: crate::OptionableConvert::try_from_optioned(
                value.non_resource_urls,
            )?,
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
    fn merge(&mut self, other: PolicyRuleAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.api_groups, other.api_groups)?;
        crate::OptionableConvert::merge(
            &mut self.non_resource_urls,
            other.non_resource_urls,
        )?;
        crate::OptionableConvert::merge(&mut self.resource_names, other.resource_names)?;
        crate::OptionableConvert::merge(&mut self.resources, other.resources)?;
        if let Some(other_value) = other.verbs {
            crate::OptionableConvert::merge(&mut self.verbs, other_value)?;
        }
        Ok(())
    }
}
