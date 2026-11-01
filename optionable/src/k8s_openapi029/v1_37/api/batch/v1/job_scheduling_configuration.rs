#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// JobSchedulingConfiguration composes the reusable workload-aware scheduling building blocks.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct JobSchedulingConfigurationAc {
    /// DisruptionMode defines the mode in which the Job's pods can be disrupted. One of Single, All. This field is immutable after creation: it may not be added or removed, and the selected mode may not be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disruption_mode: Option<
        <::k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupDisruptionMode as crate::Optionable>::Optioned,
    >,
    /// ResourceClaims defines which ResourceClaims may be shared among Pods in the Job. Pods consume the devices allocated to a PodGroup's claim by defining a claim in its own Spec.ResourceClaims that matches the PodGroup's claim exactly. The claim must have the same name and refer to the same ResourceClaim or ResourceClaimTemplate. At most 4 claims may be set, matching the limit on the resulting PodGroup. This list is immutable after creation: entries may neither be added, removed, nor modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claims: Option<
        std::vec::Vec<
            <::k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupResourceClaim as crate::Optionable>::Optioned,
        >,
    >,
    /// SchedulingConstraints defines scheduling constraints (e.g. topology) for the Job's pods. This field is immutable after creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_constraints: Option<
        <::k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingConstraints as crate::Optionable>::Optioned,
    >,
    /// SchedulingPolicy defines the scheduling policy for this Job. Exactly one of Basic or Gang must be set. This field is immutable after creation: the policy may not be added or removed. The policy variant (basic/gang) is frozen by hand-written validation; only schedulingPolicy.gang.minCount may be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_policy: Option<
        <::k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingPolicy as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::batch::v1::JobSchedulingConfiguration {
    type Optioned = JobSchedulingConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for JobSchedulingConfigurationAc {
    type Optioned = JobSchedulingConfigurationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::batch::v1::JobSchedulingConfiguration {
    fn into_optioned(self) -> JobSchedulingConfigurationAc {
        JobSchedulingConfigurationAc {
            disruption_mode: crate::OptionableConvert::into_optioned(
                self.disruption_mode,
            ),
            resource_claims: crate::OptionableConvert::into_optioned(
                self.resource_claims,
            ),
            scheduling_constraints: crate::OptionableConvert::into_optioned(
                self.scheduling_constraints,
            ),
            scheduling_policy: crate::OptionableConvert::into_optioned(
                self.scheduling_policy,
            ),
        }
    }
    fn try_from_optioned(
        value: JobSchedulingConfigurationAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            disruption_mode: crate::OptionableConvert::try_from_optioned(
                value.disruption_mode,
            )?,
            resource_claims: crate::OptionableConvert::try_from_optioned(
                value.resource_claims,
            )?,
            scheduling_constraints: crate::OptionableConvert::try_from_optioned(
                value.scheduling_constraints,
            )?,
            scheduling_policy: crate::OptionableConvert::try_from_optioned(
                value.scheduling_policy,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: JobSchedulingConfigurationAc,
    ) -> Result<(), crate::Error> {
        if self.disruption_mode.is_none() {
            self.disruption_mode = crate::OptionableConvert::try_from_optioned(
                other.disruption_mode,
            )?;
        } else if let Some(self_value) = self.disruption_mode.as_mut()
            && let Some(other_value) = other.disruption_mode
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.resource_claims.is_none() {
            self.resource_claims = crate::OptionableConvert::try_from_optioned(
                other.resource_claims,
            )?;
        } else if let Some(self_value) = self.resource_claims.as_mut()
            && let Some(other_value) = other.resource_claims
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if self.scheduling_constraints.is_none() {
            self.scheduling_constraints = crate::OptionableConvert::try_from_optioned(
                other.scheduling_constraints,
            )?;
        } else if let Some(self_value) = self.scheduling_constraints.as_mut()
            && let Some(other_value) = other.scheduling_constraints
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.scheduling_policy.is_none() {
            self.scheduling_policy = crate::OptionableConvert::try_from_optioned(
                other.scheduling_policy,
            )?;
        } else if let Some(self_value) = self.scheduling_policy.as_mut()
            && let Some(other_value) = other.scheduling_policy
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::batch::v1::JobSchedulingConfiguration>
for JobSchedulingConfigurationAc {
    fn from_optionable(
        value: k8s_openapi029::api::batch::v1::JobSchedulingConfiguration,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::batch::v1::JobSchedulingConfiguration,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::batch::v1::JobSchedulingConfiguration,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for JobSchedulingConfigurationAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.disruption_mode,
            other.disruption_mode,
        );
        crate::k8s_openapi::merge::merge_map_option_wrapped(
            &mut self.resource_claims,
            other.resource_claims,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.scheduling_constraints,
            other.scheduling_constraints,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.scheduling_policy,
            other.scheduling_policy,
        );
    }
}
