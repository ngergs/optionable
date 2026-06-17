#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// WorkloadPodGroupTemplateReference references the PodGroupTemplate within the Workload object.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkloadPodGroupTemplateReferenceAc {
    /// PodGroupTemplateName defines the PodGroupTemplate name within the Workload object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_group_template_name: Option<std::string::String>,
    /// WorkloadName defines the name of the Workload object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi028::api::scheduling::v1alpha2::WorkloadPodGroupTemplateReference {
    type Optioned = WorkloadPodGroupTemplateReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for WorkloadPodGroupTemplateReferenceAc {
    type Optioned = WorkloadPodGroupTemplateReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi028::api::scheduling::v1alpha2::WorkloadPodGroupTemplateReference {
    fn into_optioned(self) -> WorkloadPodGroupTemplateReferenceAc {
        WorkloadPodGroupTemplateReferenceAc {
            pod_group_template_name: Some(self.pod_group_template_name),
            workload_name: Some(self.workload_name),
        }
    }
    fn try_from_optioned(
        value: WorkloadPodGroupTemplateReferenceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            pod_group_template_name: value
                .pod_group_template_name
                .ok_or(crate::Error {
                    missing_field: "pod_group_template_name",
                })?,
            workload_name: value
                .workload_name
                .ok_or(crate::Error {
                    missing_field: "workload_name",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: WorkloadPodGroupTemplateReferenceAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.pod_group_template_name {
            self.pod_group_template_name = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if let Some(other_value) = other.workload_name {
            self.workload_name = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi028::api::scheduling::v1alpha2::WorkloadPodGroupTemplateReference,
> for WorkloadPodGroupTemplateReferenceAc {
    fn from_optionable(
        value: k8s_openapi028::api::scheduling::v1alpha2::WorkloadPodGroupTemplateReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi028::api::scheduling::v1alpha2::WorkloadPodGroupTemplateReference,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::api::scheduling::v1alpha2::WorkloadPodGroupTemplateReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::DeepMerge for WorkloadPodGroupTemplateReferenceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.pod_group_template_name,
            other.pod_group_template_name,
        );
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.workload_name,
            other.workload_name,
        );
    }
}
