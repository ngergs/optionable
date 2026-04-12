#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodDisruptionBudgetSpec is a description of a PodDisruptionBudget.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodDisruptionBudgetSpecAc {
    /// An eviction is allowed if at most "maxUnavailable" pods selected by "selector" are unavailable after the eviction, i.e. even in absence of the evicted pod. For example, one can prevent all voluntary evictions by specifying 0. This is a mutually exclusive setting with "minAvailable".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<
        <::k8s_openapi027::apimachinery::pkg::util::intstr::IntOrString as crate::Optionable>::Optioned,
    >,
    /// An eviction is allowed if at least "minAvailable" pods selected by "selector" will still be available after the eviction, i.e. even in the absence of the evicted pod.  So for example you can prevent all voluntary evictions by specifying "100%".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_available: Option<
        <::k8s_openapi027::apimachinery::pkg::util::intstr::IntOrString as crate::Optionable>::Optioned,
    >,
    /// Label query over pods whose evictions are managed by the disruption budget. A null selector will match no pods, while an empty ({}) selector will select all pods within the namespace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    /// UnhealthyPodEvictionPolicy defines the criteria for when unhealthy pods should be considered for eviction. Current implementation considers healthy pods, as pods that have status.conditions item with type="Ready",status="True".
    ///
    /// Valid policies are IfHealthyBudget and AlwaysAllow. If no policy is specified, the default behavior will be used, which corresponds to the IfHealthyBudget policy.
    ///
    /// IfHealthyBudget policy means that running pods (status.phase="Running"), but not yet healthy can be evicted only if the guarded application is not disrupted (status.currentHealthy is at least equal to status.desiredHealthy). Healthy pods will be subject to the PDB for eviction.
    ///
    /// AlwaysAllow policy means that all running pods (status.phase="Running"), but not yet healthy are considered disrupted and can be evicted regardless of whether the criteria in a PDB is met. This means perspective running pods of a disrupted application might not get a chance to become healthy. Healthy pods will be subject to the PDB for eviction.
    ///
    /// Additional policies may be added in the future. Clients making eviction decisions should disallow eviction of unhealthy pods if they encounter an unrecognized policy in this field.
    ///
    /// This field is beta-level. The eviction API uses this field when the feature gate PDBUnhealthyPodEvictionPolicy is enabled (enabled by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_pod_eviction_policy: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::policy::v1::PodDisruptionBudgetSpec {
    type Optioned = PodDisruptionBudgetSpecAc;
}
#[automatically_derived]
impl crate::Optionable for PodDisruptionBudgetSpecAc {
    type Optioned = PodDisruptionBudgetSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::policy::v1::PodDisruptionBudgetSpec {
    fn into_optioned(self) -> PodDisruptionBudgetSpecAc {
        PodDisruptionBudgetSpecAc {
            max_unavailable: crate::OptionableConvert::into_optioned(
                self.max_unavailable,
            ),
            min_available: crate::OptionableConvert::into_optioned(self.min_available),
            selector: crate::OptionableConvert::into_optioned(self.selector),
            unhealthy_pod_eviction_policy: self.unhealthy_pod_eviction_policy,
        }
    }
    fn try_from_optioned(
        value: PodDisruptionBudgetSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            max_unavailable: crate::OptionableConvert::try_from_optioned(
                value.max_unavailable,
            )?,
            min_available: crate::OptionableConvert::try_from_optioned(
                value.min_available,
            )?,
            selector: crate::OptionableConvert::try_from_optioned(value.selector)?,
            unhealthy_pod_eviction_policy: value.unhealthy_pod_eviction_policy,
        })
    }
    fn merge(&mut self, other: PodDisruptionBudgetSpecAc) -> Result<(), crate::Error> {
        if self.max_unavailable.is_none() {
            self.max_unavailable = crate::OptionableConvert::try_from_optioned(
                other.max_unavailable,
            )?;
        } else if let Some(self_value) = self.max_unavailable.as_mut()
            && let Some(other_value) = other.max_unavailable
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.min_available.is_none() {
            self.min_available = crate::OptionableConvert::try_from_optioned(
                other.min_available,
            )?;
        } else if let Some(self_value) = self.min_available.as_mut()
            && let Some(other_value) = other.min_available
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.selector.is_none() {
            self.selector = crate::OptionableConvert::try_from_optioned(other.selector)?;
        } else if let Some(self_value) = self.selector.as_mut()
            && let Some(other_value) = other.selector
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.unhealthy_pod_eviction_policy.is_none() {
            self.unhealthy_pod_eviction_policy = crate::OptionableConvert::try_from_optioned(
                other.unhealthy_pod_eviction_policy,
            )?;
        } else if let Some(self_value) = self.unhealthy_pod_eviction_policy.as_mut()
            && let Some(other_value) = other.unhealthy_pod_eviction_policy
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::policy::v1::PodDisruptionBudgetSpec>
for PodDisruptionBudgetSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::policy::v1::PodDisruptionBudgetSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::policy::v1::PodDisruptionBudgetSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::policy::v1::PodDisruptionBudgetSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
