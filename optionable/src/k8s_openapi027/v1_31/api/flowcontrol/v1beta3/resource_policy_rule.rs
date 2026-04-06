#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourcePolicyRuleAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_groups: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_scope: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbs: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::flowcontrol::v1beta3::ResourcePolicyRule {
    type Optioned = ResourcePolicyRuleAc;
}
#[automatically_derived]
impl crate::Optionable for ResourcePolicyRuleAc {
    type Optioned = ResourcePolicyRuleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::flowcontrol::v1beta3::ResourcePolicyRule {
    fn into_optioned(self) -> ResourcePolicyRuleAc {
        ResourcePolicyRuleAc {
            api_groups: Some(self.api_groups),
            cluster_scope: self.cluster_scope,
            namespaces: self.namespaces,
            resources: Some(self.resources),
            verbs: Some(self.verbs),
        }
    }
    fn try_from_optioned(value: ResourcePolicyRuleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_groups: value
                .api_groups
                .ok_or(crate::Error {
                    missing_field: "api_groups",
                })?,
            cluster_scope: value.cluster_scope,
            namespaces: value.namespaces,
            resources: value
                .resources
                .ok_or(crate::Error {
                    missing_field: "resources",
                })?,
            verbs: value
                .verbs
                .ok_or(crate::Error {
                    missing_field: "verbs",
                })?,
        })
    }
    fn merge(&mut self, other: ResourcePolicyRuleAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.api_groups {
            self.api_groups = other_value;
        }
        self.cluster_scope = other.cluster_scope;
        self.namespaces = other.namespaces;
        if let Some(other_value) = other.resources {
            self.resources = other_value;
        }
        if let Some(other_value) = other.verbs {
            self.verbs = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::flowcontrol::v1beta3::ResourcePolicyRule,
> for ResourcePolicyRuleAc {
    fn from_optionable(
        value: k8s_openapi027::api::flowcontrol::v1beta3::ResourcePolicyRule,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::flowcontrol::v1beta3::ResourcePolicyRule,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::flowcontrol::v1beta3::ResourcePolicyRule,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
