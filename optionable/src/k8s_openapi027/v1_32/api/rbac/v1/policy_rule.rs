#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PolicyRuleAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_groups: Option<std::vec::Vec<std::string::String>>,
    #[serde(rename = "nonResourceURLs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_resource_urls: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_names: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbs: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::rbac::v1::PolicyRule {
    type Optioned = PolicyRuleAc;
}
#[automatically_derived]
impl crate::Optionable for PolicyRuleAc {
    type Optioned = PolicyRuleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::rbac::v1::PolicyRule {
    fn into_optioned(self) -> PolicyRuleAc {
        PolicyRuleAc {
            api_groups: self.api_groups,
            non_resource_urls: self.non_resource_urls,
            resource_names: self.resource_names,
            resources: self.resources,
            verbs: Some(self.verbs),
        }
    }
    fn try_from_optioned(value: PolicyRuleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_groups: value.api_groups,
            non_resource_urls: value.non_resource_urls,
            resource_names: value.resource_names,
            resources: value.resources,
            verbs: value
                .verbs
                .ok_or(crate::Error {
                    missing_field: "verbs",
                })?,
        })
    }
    fn merge(&mut self, other: PolicyRuleAc) -> Result<(), crate::Error> {
        self.api_groups = other.api_groups;
        self.non_resource_urls = other.non_resource_urls;
        self.resource_names = other.resource_names;
        self.resources = other.resources;
        if let Some(other_value) = other.verbs {
            self.verbs = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::rbac::v1::PolicyRule> for PolicyRuleAc {
    fn from_optionable(value: k8s_openapi027::api::rbac::v1::PolicyRule) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::rbac::v1::PolicyRule, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::rbac::v1::PolicyRule,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
