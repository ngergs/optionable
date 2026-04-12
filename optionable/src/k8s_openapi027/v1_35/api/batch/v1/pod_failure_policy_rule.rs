#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodFailurePolicyRule describes how a pod failure is handled when the requirements are met. One of onExitCodes and onPodConditions, but not both, can be used in each rule.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodFailurePolicyRuleAc {
    /// Specifies the action taken on a pod failure when the requirements are satisfied. Possible values are:
    ///
    /// - FailJob: indicates that the pod's job is marked as Failed and all
    ///   running pods are terminated.
    /// - FailIndex: indicates that the pod's index is marked as Failed and will
    ///   not be restarted.
    /// - Ignore: indicates that the counter towards the .backoffLimit is not
    ///   incremented and a replacement pod is created.
    /// - Count: indicates that the pod is handled in the default way - the
    ///   counter towards the .backoffLimit is incremented.
    /// Additional values are considered to be added in the future. Clients should react to an unknown action by skipping the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<std::string::String>,
    /// Represents the requirement on the container exit codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_exit_codes: Option<
        <::k8s_openapi027::api::batch::v1::PodFailurePolicyOnExitCodesRequirement as crate::Optionable>::Optioned,
    >,
    /// Represents the requirement on the pod conditions. The requirement is represented as a list of pod condition patterns. The requirement is satisfied if at least one pattern matches an actual pod condition. At most 20 elements are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_pod_conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::batch::v1::PodFailurePolicyOnPodConditionsPattern as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::batch::v1::PodFailurePolicyRule {
    type Optioned = PodFailurePolicyRuleAc;
}
#[automatically_derived]
impl crate::Optionable for PodFailurePolicyRuleAc {
    type Optioned = PodFailurePolicyRuleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::batch::v1::PodFailurePolicyRule {
    fn into_optioned(self) -> PodFailurePolicyRuleAc {
        PodFailurePolicyRuleAc {
            action: Some(self.action),
            on_exit_codes: crate::OptionableConvert::into_optioned(self.on_exit_codes),
            on_pod_conditions: crate::OptionableConvert::into_optioned(
                self.on_pod_conditions,
            ),
        }
    }
    fn try_from_optioned(value: PodFailurePolicyRuleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            action: value
                .action
                .ok_or(crate::Error {
                    missing_field: "action",
                })?,
            on_exit_codes: crate::OptionableConvert::try_from_optioned(
                value.on_exit_codes,
            )?,
            on_pod_conditions: crate::OptionableConvert::try_from_optioned(
                value.on_pod_conditions,
            )?,
        })
    }
    fn merge(&mut self, other: PodFailurePolicyRuleAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.action {
            self.action = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.on_exit_codes.is_none() {
            self.on_exit_codes = crate::OptionableConvert::try_from_optioned(
                other.on_exit_codes,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.on_exit_codes,
                other.on_exit_codes,
            )?;
        }
        if self.on_pod_conditions.is_none() {
            self.on_pod_conditions = crate::OptionableConvert::try_from_optioned(
                other.on_pod_conditions,
            )?;
        } else {
            self.on_pod_conditions = crate::OptionableConvert::try_from_optioned(
                other.on_pod_conditions,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::batch::v1::PodFailurePolicyRule>
for PodFailurePolicyRuleAc {
    fn from_optionable(
        value: k8s_openapi027::api::batch::v1::PodFailurePolicyRule,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::batch::v1::PodFailurePolicyRule, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::batch::v1::PodFailurePolicyRule,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
