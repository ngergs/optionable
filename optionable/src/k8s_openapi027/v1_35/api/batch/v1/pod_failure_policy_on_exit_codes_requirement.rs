#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodFailurePolicyOnExitCodesRequirement describes the requirement for handling a failed pod based on its container exit codes. In particular, it lookups the .state.terminated.exitCode for each app container and init container status, represented by the .status.containerStatuses and .status.initContainerStatuses fields in the Pod status, respectively. Containers completed with success (exit code 0) are excluded from the requirement check.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodFailurePolicyOnExitCodesRequirementAc {
    /// Restricts the check for exit codes to the container with the specified name. When null, the rule applies to all containers. When specified, it should match one the container or initContainer names in the pod template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<std::string::String>,
    /// Represents the relationship between the container exit code(s) and the specified values. Containers completed with success (exit code 0) are excluded from the requirement check. Possible values are:
    ///
    /// - In: the requirement is satisfied if at least one container exit code
    ///   (might be multiple if there are multiple containers not restricted
    ///   by the 'containerName' field) is in the set of specified values.
    /// - NotIn: the requirement is satisfied if at least one container exit code
    ///   (might be multiple if there are multiple containers not restricted
    ///   by the 'containerName' field) is not in the set of specified values.
    /// Additional values are considered to be added in the future. Clients should react to an unknown operator by assuming the requirement is not satisfied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<std::string::String>,
    /// Specifies the set of values. Each returned container exit code (might be multiple in case of multiple containers) is checked against this set of values with respect to the operator. The list of values must be ordered and must not contain duplicates. Value '0' cannot be used for the In operator. At least one element is required. At most 255 elements are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<std::vec::Vec<i32>>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::batch::v1::PodFailurePolicyOnExitCodesRequirement {
    type Optioned = PodFailurePolicyOnExitCodesRequirementAc;
}
#[automatically_derived]
impl crate::Optionable for PodFailurePolicyOnExitCodesRequirementAc {
    type Optioned = PodFailurePolicyOnExitCodesRequirementAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::batch::v1::PodFailurePolicyOnExitCodesRequirement {
    fn into_optioned(self) -> PodFailurePolicyOnExitCodesRequirementAc {
        PodFailurePolicyOnExitCodesRequirementAc {
            container_name: self.container_name,
            operator: Some(self.operator),
            values: Some(self.values),
        }
    }
    fn try_from_optioned(
        value: PodFailurePolicyOnExitCodesRequirementAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            container_name: value.container_name,
            operator: value
                .operator
                .ok_or(crate::Error {
                    missing_field: "operator",
                })?,
            values: value
                .values
                .ok_or(crate::Error {
                    missing_field: "values",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: PodFailurePolicyOnExitCodesRequirementAc,
    ) -> Result<(), crate::Error> {
        if self.container_name.is_none() {
            self.container_name = crate::OptionableConvert::try_from_optioned(
                other.container_name,
            )?;
        } else if let Some(self_value) = self.container_name.as_mut()
            && let Some(other_value) = other.container_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.operator {
            self.operator = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.values {
            crate::merge::try_merge_optioned_set(&mut self.values, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::batch::v1::PodFailurePolicyOnExitCodesRequirement,
> for PodFailurePolicyOnExitCodesRequirementAc {
    fn from_optionable(
        value: k8s_openapi027::api::batch::v1::PodFailurePolicyOnExitCodesRequirement,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::batch::v1::PodFailurePolicyOnExitCodesRequirement,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::batch::v1::PodFailurePolicyOnExitCodesRequirement,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for PodFailurePolicyOnExitCodesRequirementAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.container_name,
            other.container_name,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.operator, other.operator);
        crate::merge::merge_append_not_present_option_wrapped(
            &mut self.values,
            other.values,
        );
    }
}
