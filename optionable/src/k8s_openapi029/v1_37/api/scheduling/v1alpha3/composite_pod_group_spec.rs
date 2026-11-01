#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CompositePodGroupSpec defines the desired state of CompositePodGroup.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CompositePodGroupSpecAc {
    /// disruptionMode defines the mode in which a given CompositePodGroup can be disrupted. Controllers are expected to fill this field by copying it from a CompositePodGroupTemplate. One of Single, All. Defaults to Single if unset. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disruption_mode: Option<
        <::k8s_openapi029::api::scheduling::v1alpha3::CompositeDisruptionMode as crate::Optionable>::Optioned,
    >,
    /// parentCompositePodGroupName contains the name of the parent composite pod group within the same namespace as this composite pod group. It must be a DNS name. If it's nil, then this composite pod group is a root of a workload's hierarchy. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_composite_pod_group_name: Option<std::string::String>,
    /// preemptionPolicy is the Policy for preempting pods/podgroups with lower priority. One of Never, PreemptLowerPriority. Defaults to PreemptLowerPriority if unset. When Priority Admission Controller is enabled, it populates this field from PriorityClassName, and defaults to PreemptLowerPriority if value is unset in PriorityClass. This field is immutable. This field is available only when the PodGroupPreemptionPolicy feature gate is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preemption_policy: Option<std::string::String>,
    /// priority is the value of priority of this composite pod group. Various system components use this field to find the priority of the composite pod group. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// priorityClassName defines the priority that should be considered when scheduling this CompositePodGroup. Controllers are expected to fill this field by copying it from a CompositePodGroupTemplate. If left unspecified, it is validated and resolved similarly to the PriorityClassName field in Pods (i.e. if no priority class is specified, admission control can set this to the global default priority class if it exists. Otherwise, the composite pod group's priority will be zero). This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_class_name: Option<std::string::String>,
    /// schedulingConstraints defines optional scheduling constraints (e.g. topology) for this CompositePodGroup. Controllers are expected to fill this field by copying it from a CompositePodGroupTemplate. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_constraints: Option<
        <::k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupSchedulingConstraints as crate::Optionable>::Optioned,
    >,
    /// schedulingPolicy defines the scheduling policy for this instance of the CompositePodGroup. Controllers are expected to fill this field by copying it from a CompositePodGroupTemplate. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_policy: Option<
        <::k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupSchedulingPolicy as crate::Optionable>::Optioned,
    >,
    /// workloadRef references an optional CompositePodGroup template within the Workload object that was used to create the CompositePodGroup. This field is required. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_ref: Option<
        <::k8s_openapi029::api::scheduling::v1alpha3::WorkloadReference as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupSpec {
    type Optioned = CompositePodGroupSpecAc;
}
#[automatically_derived]
impl crate::Optionable for CompositePodGroupSpecAc {
    type Optioned = CompositePodGroupSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupSpec {
    fn into_optioned(self) -> CompositePodGroupSpecAc {
        CompositePodGroupSpecAc {
            disruption_mode: crate::OptionableConvert::into_optioned(
                self.disruption_mode,
            ),
            parent_composite_pod_group_name: self.parent_composite_pod_group_name,
            preemption_policy: self.preemption_policy,
            priority: self.priority,
            priority_class_name: self.priority_class_name,
            scheduling_constraints: crate::OptionableConvert::into_optioned(
                self.scheduling_constraints,
            ),
            scheduling_policy: Some(
                crate::OptionableConvert::into_optioned(self.scheduling_policy),
            ),
            workload_ref: Some(
                crate::OptionableConvert::into_optioned(self.workload_ref),
            ),
        }
    }
    fn try_from_optioned(value: CompositePodGroupSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            disruption_mode: crate::OptionableConvert::try_from_optioned(
                value.disruption_mode,
            )?,
            parent_composite_pod_group_name: value.parent_composite_pod_group_name,
            preemption_policy: value.preemption_policy,
            priority: value.priority,
            priority_class_name: value.priority_class_name,
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
            workload_ref: crate::OptionableConvert::try_from_optioned(
                value
                    .workload_ref
                    .ok_or(crate::Error {
                        missing_field: "workload_ref",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: CompositePodGroupSpecAc) -> Result<(), crate::Error> {
        if self.disruption_mode.is_none() {
            self.disruption_mode = crate::OptionableConvert::try_from_optioned(
                other.disruption_mode,
            )?;
        } else if let Some(self_value) = self.disruption_mode.as_mut()
            && let Some(other_value) = other.disruption_mode
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.parent_composite_pod_group_name.is_none() {
            self.parent_composite_pod_group_name = crate::OptionableConvert::try_from_optioned(
                other.parent_composite_pod_group_name,
            )?;
        } else if let Some(self_value) = self.parent_composite_pod_group_name.as_mut()
            && let Some(other_value) = other.parent_composite_pod_group_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
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
        if let Some(other_value) = other.workload_ref {
            crate::OptionableConvert::merge(&mut self.workload_ref, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupSpec,
> for CompositePodGroupSpecAc {
    fn from_optionable(
        value: k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for CompositePodGroupSpecAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.disruption_mode,
            other.disruption_mode,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.parent_composite_pod_group_name,
            other.parent_composite_pod_group_name,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.preemption_policy,
            other.preemption_policy,
        );
        k8s_openapi029::DeepMerge::merge_from(&mut self.priority, other.priority);
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.priority_class_name,
            other.priority_class_name,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.scheduling_constraints,
            other.scheduling_constraints,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.scheduling_policy,
            other.scheduling_policy,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.workload_ref,
            other.workload_ref,
        );
    }
}
