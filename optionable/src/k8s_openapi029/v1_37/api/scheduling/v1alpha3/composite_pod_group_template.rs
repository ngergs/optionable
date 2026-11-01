#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CompositePodGroupTemplate represents a template for a CompositePodGroup with a scheduling policy.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CompositePodGroupTemplateAc {
    /// compositePodGroupTemplates is the list of templates for children CompositePodGroups. The maximum number of templates is 8. At least one entry in CompositePodGroupTemplates or PodGroupTemplates must be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_pod_group_templates: Option<
        std::vec::Vec<
            <::k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupTemplate as crate::Optionable>::Optioned,
        >,
    >,
    /// disruptionMode defines the mode in which a given CompositePodGroup can be disrupted. One of Single, All. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disruption_mode: Option<
        <::k8s_openapi029::api::scheduling::v1alpha3::CompositeDisruptionMode as crate::Optionable>::Optioned,
    >,
    /// name is a unique identifier for the CompositePodGroupTemplate within the Workload. It must be a DNS label. This field is required.
    pub name: std::string::String,
    /// podGroupTemplates is the list of templates for children PodGroups. The maximum number of templates is 8. At least one entry in CompositePodGroupTemplates or PodGroupTemplates must be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_group_templates: Option<
        std::vec::Vec<
            <::k8s_openapi029::api::scheduling::v1alpha3::PodGroupTemplate as crate::Optionable>::Optioned,
        >,
    >,
    /// preemptionPolicy is the Policy for preempting pods/podgroups with lower priority. One of Never, PreemptLowerPriority. This field is immutable. This field is available only when the PodGroupPreemptionPolicy feature gate is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preemption_policy: Option<std::string::String>,
    /// priority is the value of priority of composite pod groups created from this template. Various system components use this field to find the priority of the composite pod group. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// priorityClassName indicates the priority that should be considered when scheduling a composite pod group created from this template. If no priority class is specified, admission control can set this to the global default priority class if it exists. Otherwise, composite pod groups created from this template will have the priority set to zero. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_class_name: Option<std::string::String>,
    /// schedulingConstraints defines optional scheduling constraints (e.g. topology) for this CompositePodGroupTemplate. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_constraints: Option<
        <::k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupSchedulingConstraints as crate::Optionable>::Optioned,
    >,
    /// schedulingPolicy defines the scheduling policy for this template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_policy: Option<
        <::k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupSchedulingPolicy as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupTemplate {
    type Optioned = CompositePodGroupTemplateAc;
}
#[automatically_derived]
impl crate::Optionable for CompositePodGroupTemplateAc {
    type Optioned = CompositePodGroupTemplateAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupTemplate {
    fn into_optioned(self) -> CompositePodGroupTemplateAc {
        CompositePodGroupTemplateAc {
            composite_pod_group_templates: crate::OptionableConvert::into_optioned(
                self.composite_pod_group_templates,
            ),
            disruption_mode: crate::OptionableConvert::into_optioned(
                self.disruption_mode,
            ),
            name: self.name,
            pod_group_templates: crate::OptionableConvert::into_optioned(
                self.pod_group_templates,
            ),
            preemption_policy: self.preemption_policy,
            priority: self.priority,
            priority_class_name: self.priority_class_name,
            scheduling_constraints: crate::OptionableConvert::into_optioned(
                self.scheduling_constraints,
            ),
            scheduling_policy: Some(
                crate::OptionableConvert::into_optioned(self.scheduling_policy),
            ),
        }
    }
    fn try_from_optioned(
        value: CompositePodGroupTemplateAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            composite_pod_group_templates: crate::OptionableConvert::try_from_optioned(
                value.composite_pod_group_templates,
            )?,
            disruption_mode: crate::OptionableConvert::try_from_optioned(
                value.disruption_mode,
            )?,
            name: value.name,
            pod_group_templates: crate::OptionableConvert::try_from_optioned(
                value.pod_group_templates,
            )?,
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
        })
    }
    fn merge(&mut self, other: CompositePodGroupTemplateAc) -> Result<(), crate::Error> {
        if self.composite_pod_group_templates.is_none() {
            self.composite_pod_group_templates = crate::OptionableConvert::try_from_optioned(
                other.composite_pod_group_templates,
            )?;
        } else if let Some(self_value) = self.composite_pod_group_templates.as_mut()
            && let Some(other_value) = other.composite_pod_group_templates
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
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
        if self.pod_group_templates.is_none() {
            self.pod_group_templates = crate::OptionableConvert::try_from_optioned(
                other.pod_group_templates,
            )?;
        } else if let Some(self_value) = self.pod_group_templates.as_mut()
            && let Some(other_value) = other.pod_group_templates
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
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
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupTemplate {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.name == other.name
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupTemplate,
> for CompositePodGroupTemplateAc {
    fn from_optionable(
        value: k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupTemplate,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupTemplate,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupTemplate,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for CompositePodGroupTemplateAc {
    fn merge_from(&mut self, other: Self) {
        crate::k8s_openapi::merge::merge_map_option_wrapped(
            &mut self.composite_pod_group_templates,
            other.composite_pod_group_templates,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.disruption_mode,
            other.disruption_mode,
        );
        k8s_openapi029::DeepMerge::merge_from(&mut self.name, other.name);
        crate::k8s_openapi::merge::merge_map_option_wrapped(
            &mut self.pod_group_templates,
            other.pod_group_templates,
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
    }
}
impl crate::merge::MapKeysEq for CompositePodGroupTemplateAc {
    fn keys_eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
