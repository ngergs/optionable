#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// WorkloadSpec defines the desired state of a Workload.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkloadSpecAc {
    /// compositePodGroupTemplates is the list of CompositePodGroup templates that make up the Workload. The maximum number of templates is 8. This field is immutable. Exactly one of CompositePodGroupTemplates and PodGroupTemplates must be set.
    ///
    /// This field is used only when the CompositePodGroup feature gate is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_pod_group_templates: Option<
        std::vec::Vec<
            <::k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupTemplate as crate::Optionable>::Optioned,
        >,
    >,
    /// controllerRef is an optional reference to the controlling object, such as a Deployment or Job. This field is intended for use by tools like CLIs to provide a link back to the original workload definition. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_ref: Option<
        <::k8s_openapi029::api::scheduling::v1alpha3::TypedLocalObjectReference as crate::Optionable>::Optioned,
    >,
    /// podGroupTemplates is the list of templates that make up the Workload. The maximum number of templates is 8. Templates cannot be added or removed after the workload is created. Existing templates may still be updated where their individual fields allow it. Exactly one of CompositePodGroupTemplates and PodGroupTemplates must be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_group_templates: Option<
        std::vec::Vec<
            <::k8s_openapi029::api::scheduling::v1alpha3::PodGroupTemplate as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::scheduling::v1alpha3::WorkloadSpec {
    type Optioned = WorkloadSpecAc;
}
#[automatically_derived]
impl crate::Optionable for WorkloadSpecAc {
    type Optioned = WorkloadSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::scheduling::v1alpha3::WorkloadSpec {
    fn into_optioned(self) -> WorkloadSpecAc {
        WorkloadSpecAc {
            composite_pod_group_templates: crate::OptionableConvert::into_optioned(
                self.composite_pod_group_templates,
            ),
            controller_ref: crate::OptionableConvert::into_optioned(self.controller_ref),
            pod_group_templates: crate::OptionableConvert::into_optioned(
                self.pod_group_templates,
            ),
        }
    }
    fn try_from_optioned(value: WorkloadSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            composite_pod_group_templates: crate::OptionableConvert::try_from_optioned(
                value.composite_pod_group_templates,
            )?,
            controller_ref: crate::OptionableConvert::try_from_optioned(
                value.controller_ref,
            )?,
            pod_group_templates: crate::OptionableConvert::try_from_optioned(
                value.pod_group_templates,
            )?,
        })
    }
    fn merge(&mut self, other: WorkloadSpecAc) -> Result<(), crate::Error> {
        if self.composite_pod_group_templates.is_none() {
            self.composite_pod_group_templates = crate::OptionableConvert::try_from_optioned(
                other.composite_pod_group_templates,
            )?;
        } else if let Some(self_value) = self.composite_pod_group_templates.as_mut()
            && let Some(other_value) = other.composite_pod_group_templates
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if self.controller_ref.is_none() {
            self.controller_ref = crate::OptionableConvert::try_from_optioned(
                other.controller_ref,
            )?;
        } else if let Some(self_value) = self.controller_ref.as_mut()
            && let Some(other_value) = other.controller_ref
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.pod_group_templates.is_none() {
            self.pod_group_templates = crate::OptionableConvert::try_from_optioned(
                other.pod_group_templates,
            )?;
        } else if let Some(self_value) = self.pod_group_templates.as_mut()
            && let Some(other_value) = other.pod_group_templates
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::scheduling::v1alpha3::WorkloadSpec>
for WorkloadSpecAc {
    fn from_optionable(
        value: k8s_openapi029::api::scheduling::v1alpha3::WorkloadSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi029::api::scheduling::v1alpha3::WorkloadSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::scheduling::v1alpha3::WorkloadSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for WorkloadSpecAc {
    fn merge_from(&mut self, other: Self) {
        crate::k8s_openapi::merge::merge_map_option_wrapped(
            &mut self.composite_pod_group_templates,
            other.composite_pod_group_templates,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.controller_ref,
            other.controller_ref,
        );
        crate::k8s_openapi::merge::merge_map_option_wrapped(
            &mut self.pod_group_templates,
            other.pod_group_templates,
        );
    }
}
