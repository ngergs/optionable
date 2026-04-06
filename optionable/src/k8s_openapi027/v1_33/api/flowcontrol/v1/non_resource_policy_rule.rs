#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NonResourcePolicyRuleAc {
    #[serde(rename = "nonResourceURLs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_resource_urls: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbs: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::flowcontrol::v1::NonResourcePolicyRule {
    type Optioned = NonResourcePolicyRuleAc;
}
#[automatically_derived]
impl crate::Optionable for NonResourcePolicyRuleAc {
    type Optioned = NonResourcePolicyRuleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::flowcontrol::v1::NonResourcePolicyRule {
    fn into_optioned(self) -> NonResourcePolicyRuleAc {
        NonResourcePolicyRuleAc {
            non_resource_urls: Some(self.non_resource_urls),
            verbs: Some(self.verbs),
        }
    }
    fn try_from_optioned(value: NonResourcePolicyRuleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            non_resource_urls: value
                .non_resource_urls
                .ok_or(crate::Error {
                    missing_field: "non_resource_urls",
                })?,
            verbs: value
                .verbs
                .ok_or(crate::Error {
                    missing_field: "verbs",
                })?,
        })
    }
    fn merge(&mut self, other: NonResourcePolicyRuleAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.non_resource_urls {
            self.non_resource_urls = other_value;
        }
        if let Some(other_value) = other.verbs {
            self.verbs = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::flowcontrol::v1::NonResourcePolicyRule>
for NonResourcePolicyRuleAc {
    fn from_optionable(
        value: k8s_openapi027::api::flowcontrol::v1::NonResourcePolicyRule,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::flowcontrol::v1::NonResourcePolicyRule,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::flowcontrol::v1::NonResourcePolicyRule,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
