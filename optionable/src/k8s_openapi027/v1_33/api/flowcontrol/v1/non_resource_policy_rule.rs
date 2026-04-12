#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NonResourcePolicyRule is a predicate that matches non-resource requests according to their verb and the target non-resource URL. A NonResourcePolicyRule matches a request if and only if both (a) at least one member of verbs matches the request and (b) at least one member of nonResourceURLs matches the request.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NonResourcePolicyRuleAc {
    /// `nonResourceURLs` is a set of url prefixes that a user should have access to and may not be empty. For example:
    ///   - "/healthz" is legal
    ///   - "/hea*" is illegal
    ///   - "/hea" is legal but matches nothing
    ///   - "/hea/*" also matches nothing
    ///   - "/healthz/*" matches all per-component health checks.
    /// "*" matches all non-resource urls. if it is present, it must be the only entry. Required.
    #[serde(rename = "nonResourceURLs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_resource_urls: Option<std::vec::Vec<std::string::String>>,
    /// `verbs` is a list of matching verbs and may not be empty. "*" matches all verbs. If it is present, it must be the only entry. Required.
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
            crate::merge::try_merge_optioned_set(
                &mut self.non_resource_urls,
                other_value,
            )?;
        }
        if let Some(other_value) = other.verbs {
            crate::merge::try_merge_optioned_set(&mut self.verbs, other_value)?;
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
