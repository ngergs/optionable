pub struct ResourcePolicyRuleAc {
    pub api_groups: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
    pub cluster_scope: <Option<bool> as crate::Optionable>::Optioned,
    pub namespaces: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub resources: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
    pub verbs: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::flowcontrol::v1::ResourcePolicyRule {
    type Optioned = ResourcePolicyRuleAc;
}
#[automatically_derived]
impl crate::Optionable for ResourcePolicyRuleAc {
    type Optioned = ResourcePolicyRuleAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::ResourcePolicyRule {
    fn into_optioned(self) -> ResourcePolicyRuleAc {
        ResourcePolicyRuleAc {
            api_groups: Some(crate::OptionableConvert::into_optioned(self.api_groups)),
            cluster_scope: crate::OptionableConvert::into_optioned(self.cluster_scope),
            namespaces: crate::OptionableConvert::into_optioned(self.namespaces),
            resources: Some(crate::OptionableConvert::into_optioned(self.resources)),
            verbs: Some(crate::OptionableConvert::into_optioned(self.verbs)),
        }
    }
    fn try_from_optioned(
        value: ResourcePolicyRuleAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_groups: crate::OptionableConvert::try_from_optioned(
                value
                    .api_groups
                    .ok_or(crate::optionable::Error {
                        missing_field: "api_groups",
                    })?,
            )?,
            cluster_scope: crate::OptionableConvert::try_from_optioned(
                value.cluster_scope,
            )?,
            namespaces: crate::OptionableConvert::try_from_optioned(value.namespaces)?,
            resources: crate::OptionableConvert::try_from_optioned(
                value
                    .resources
                    .ok_or(crate::optionable::Error {
                        missing_field: "resources",
                    })?,
            )?,
            verbs: crate::OptionableConvert::try_from_optioned(
                value
                    .verbs
                    .ok_or(crate::optionable::Error {
                        missing_field: "verbs",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ResourcePolicyRuleAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.api_groups {
            crate::OptionableConvert::merge(&mut self.api_groups, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.cluster_scope, other.cluster_scope)?;
        crate::OptionableConvert::merge(&mut self.namespaces, other.namespaces)?;
        if let Some(other_value) = other.resources {
            crate::OptionableConvert::merge(&mut self.resources, other_value)?;
        }
        if let Some(other_value) = other.verbs {
            crate::OptionableConvert::merge(&mut self.verbs, other_value)?;
        }
        Ok(())
    }
}
