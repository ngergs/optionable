#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ResourceRule is the list of actions the subject is allowed to perform on resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceRuleAc {
    /// APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of the enumerated resources in any API group will be allowed.  "*" means all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_groups: Option<std::vec::Vec<std::string::String>>,
    /// ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.  "*" means all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_names: Option<std::vec::Vec<std::string::String>>,
    /// Resources is a list of resources this rule applies to.  "*" means all in the specified apiGroups.
    ///  "*/foo" represents the subresource 'foo' for all resources in the specified apiGroups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<std::vec::Vec<std::string::String>>,
    /// Verb is a list of kubernetes resource API verbs, like: get, list, watch, create, update, delete, proxy.  "*" means all.
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
        if self.api_groups.is_none() {
            self.api_groups = crate::OptionableConvert::try_from_optioned(
                other.api_groups,
            )?;
        } else if let Some(self_value) = self.api_groups.as_mut()
            && let Some(other_value) = other.api_groups
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.resource_names.is_none() {
            self.resource_names = crate::OptionableConvert::try_from_optioned(
                other.resource_names,
            )?;
        } else if let Some(self_value) = self.resource_names.as_mut()
            && let Some(other_value) = other.resource_names
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.resources.is_none() {
            self.resources = crate::OptionableConvert::try_from_optioned(
                other.resources,
            )?;
        } else if let Some(self_value) = self.resources.as_mut()
            && let Some(other_value) = other.resources
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.verbs {
            self.verbs = crate::OptionableConvert::try_from_optioned(other_value)?;
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
impl k8s_openapi027::DeepMerge for ResourceRuleAc {
    fn merge_from(&mut self, other: Self) {
        self.api_groups = other.api_groups;
        self.resource_names = other.resource_names;
        self.resources = other.resources;
        self.verbs = other.verbs;
    }
}
