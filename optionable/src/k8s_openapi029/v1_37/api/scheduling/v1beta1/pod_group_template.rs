#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodGroupTemplate represents a template for a set of pods with a scheduling policy.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodGroupTemplateAc {
    /// disruptionMode defines the mode in which a given PodGroup can be disrupted. One of Single, All. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disruption_mode: Option<
        <::k8s_openapi029::api::scheduling::v1beta1::DisruptionMode as crate::Optionable>::Optioned,
    >,
    /// name is a unique identifier for the PodGroupTemplate within the Workload. It must be a DNS label. This field is immutable.
    pub name: std::string::String,
    /// preemptionPolicy is the Policy for preempting pods/podgroups with lower priority. One of Never, PreemptLowerPriority. This field is immutable. This field is available only when the PodGroupPreemptionPolicy feature gate is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preemption_policy: Option<std::string::String>,
    /// priority is the value of priority of pod groups created from this template. Various system components use this field to find the priority of the pod group. The higher the value, the higher the priority. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// priorityClassName indicates the priority that should be considered when scheduling a pod group created from this template. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_class_name: Option<std::string::String>,
    /// resourceClaims defines which ResourceClaims may be shared among Pods in the group. Pods consume the devices allocated to a PodGroup's claim by defining a claim in its own Spec.ResourceClaims that matches the PodGroup's claim exactly. The claim must have the same name and refer to the same ResourceClaim or ResourceClaimTemplate.
    ///
    /// This is a beta-level field and requires that the DRAWorkloadResourceClaims feature gate is enabled.
    ///
    /// This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claims: Option<
        std::vec::Vec<
            <::k8s_openapi029::api::scheduling::v1beta1::PodGroupResourceClaim as crate::Optionable>::Optioned,
        >,
    >,
    /// schedulingConstraints defines optional scheduling constraints (e.g. topology) for this PodGroupTemplate. This field is only available when the TopologyAwareWorkloadScheduling feature gate is enabled. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_constraints: Option<
        <::k8s_openapi029::api::scheduling::v1beta1::PodGroupSchedulingConstraints as crate::Optionable>::Optioned,
    >,
    /// schedulingPolicy defines the scheduling policy for this PodGroupTemplate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_policy: Option<
        <::k8s_openapi029::api::scheduling::v1beta1::PodGroupSchedulingPolicy as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::scheduling::v1beta1::PodGroupTemplate {
    type Optioned = PodGroupTemplateAc;
}
#[automatically_derived]
impl crate::Optionable for PodGroupTemplateAc {
    type Optioned = PodGroupTemplateAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::scheduling::v1beta1::PodGroupTemplate {
    fn into_optioned(self) -> PodGroupTemplateAc {
        PodGroupTemplateAc {
            disruption_mode: crate::OptionableConvert::into_optioned(
                self.disruption_mode,
            ),
            name: self.name,
            preemption_policy: self.preemption_policy,
            priority: self.priority,
            priority_class_name: self.priority_class_name,
            resource_claims: crate::OptionableConvert::into_optioned(
                self.resource_claims,
            ),
            scheduling_constraints: crate::OptionableConvert::into_optioned(
                self.scheduling_constraints,
            ),
            scheduling_policy: Some(
                crate::OptionableConvert::into_optioned(self.scheduling_policy),
            ),
        }
    }
    fn try_from_optioned(value: PodGroupTemplateAc) -> Result<Self, crate::Error> {
        Ok(Self {
            disruption_mode: crate::OptionableConvert::try_from_optioned(
                value.disruption_mode,
            )?,
            name: value.name,
            preemption_policy: value.preemption_policy,
            priority: value.priority,
            priority_class_name: value.priority_class_name,
            resource_claims: crate::OptionableConvert::try_from_optioned(
                value.resource_claims,
            )?,
            scheduling_constraints: crate::OptionableConvert::try_from_optioned(
                value.scheduling_constraints,
            )?,
            scheduling_policy: crate::OptionableConvert::try_from_optioned(
                value
                    .scheduling_policy
                    .ok_or(crate::Error {
                        missing_field: "scheduling_policy",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: PodGroupTemplateAc) -> Result<(), crate::Error> {
        if self.disruption_mode.is_none() {
            self.disruption_mode = crate::OptionableConvert::try_from_optioned(
                other.disruption_mode,
            )?;
        } else if let Some(self_value) = self.disruption_mode.as_mut()
            && let Some(other_value) = other.disruption_mode
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        self.name = other.name;
        if self.preemption_policy.is_none() {
            self.preemption_policy = crate::OptionableConvert::try_from_optioned(
                other.preemption_policy,
            )?;
        } else if let Some(self_value) = self.preemption_policy.as_mut()
            && let Some(other_value) = other.preemption_policy
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.priority.is_none() {
            self.priority = crate::OptionableConvert::try_from_optioned(other.priority)?;
        } else if let Some(self_value) = self.priority.as_mut()
            && let Some(other_value) = other.priority
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.priority_class_name.is_none() {
            self.priority_class_name = crate::OptionableConvert::try_from_optioned(
                other.priority_class_name,
            )?;
        } else if let Some(self_value) = self.priority_class_name.as_mut()
            && let Some(other_value) = other.priority_class_name
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
        if let Some(other_value) = other.scheduling_policy {
            crate::OptionableConvert::merge(&mut self.scheduling_policy, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi029::api::scheduling::v1beta1::PodGroupTemplate {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.name == other.name
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::scheduling::v1beta1::PodGroupTemplate>
for PodGroupTemplateAc {
    fn from_optionable(
        value: k8s_openapi029::api::scheduling::v1beta1::PodGroupTemplate,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::scheduling::v1beta1::PodGroupTemplate,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::scheduling::v1beta1::PodGroupTemplate,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for PodGroupTemplateAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.disruption_mode,
            other.disruption_mode,
        );
        k8s_openapi029::DeepMerge::merge_from(&mut self.name, other.name);
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.preemption_policy,
            other.preemption_policy,
        );
        k8s_openapi029::DeepMerge::merge_from(&mut self.priority, other.priority);
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.priority_class_name,
            other.priority_class_name,
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
impl crate::merge::MapKeysEq for PodGroupTemplateAc {
    fn keys_eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
