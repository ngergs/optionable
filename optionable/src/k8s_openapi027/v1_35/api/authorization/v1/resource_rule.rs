#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceRuleAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_groups: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_names: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbs: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::authorization::v1::ResourceRule {
    type Optioned = ResourceRuleAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceRuleAc {
    type Optioned = ResourceRuleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::authorization::v1::ResourceRule {
    fn into_optioned(self) -> ResourceRuleAc {
        ResourceRuleAc {
            api_groups: self.api_groups,
            resource_names: self.resource_names,
            resources: self.resources,
            verbs: Some(self.verbs),
        }
    }
    fn try_from_optioned(value: ResourceRuleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_groups: value.api_groups,
            resource_names: value.resource_names,
            resources: value.resources,
            verbs: value
                .verbs
                .ok_or(crate::Error {
                    missing_field: "verbs",
                })?,
        })
    }
    fn merge(&mut self, other: ResourceRuleAc) -> Result<(), crate::Error> {
        self.api_groups = other.api_groups;
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
impl crate::OptionedConvert<k8s_openapi027::api::authorization::v1::ResourceRule>
for ResourceRuleAc {
    fn from_optionable(
        value: k8s_openapi027::api::authorization::v1::ResourceRule,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::authorization::v1::ResourceRule, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::authorization::v1::ResourceRule,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
