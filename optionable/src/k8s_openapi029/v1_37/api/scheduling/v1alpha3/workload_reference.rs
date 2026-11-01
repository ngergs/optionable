#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// WorkloadReference references the Workload object together with the template that was used to create a particular PodGroup.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkloadReferenceAc {
    /// templateName is the name of a template within the Workload object that was used to create a pod group. It must be a DNS label. This field is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<std::string::String>,
    /// workloadName is the name of the Workload object that contains a template that was used when creating a pod group. It must be a DNS name. This field is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::scheduling::v1alpha3::WorkloadReference {
    type Optioned = WorkloadReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for WorkloadReferenceAc {
    type Optioned = WorkloadReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::scheduling::v1alpha3::WorkloadReference {
    fn into_optioned(self) -> WorkloadReferenceAc {
        WorkloadReferenceAc {
            template_name: Some(self.template_name),
            workload_name: Some(self.workload_name),
        }
    }
    fn try_from_optioned(value: WorkloadReferenceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            template_name: value
                .template_name
                .ok_or(crate::Error {
                    missing_field: "template_name",
                })?,
            workload_name: value
                .workload_name
                .ok_or(crate::Error {
                    missing_field: "workload_name",
                })?,
        })
    }
    fn merge(&mut self, other: WorkloadReferenceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.template_name {
            self.template_name = crate::OptionableConvert::try_from_optioned(
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
impl crate::OptionedConvert<k8s_openapi029::api::scheduling::v1alpha3::WorkloadReference>
for WorkloadReferenceAc {
    fn from_optionable(
        value: k8s_openapi029::api::scheduling::v1alpha3::WorkloadReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::scheduling::v1alpha3::WorkloadReference,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::scheduling::v1alpha3::WorkloadReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for WorkloadReferenceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.template_name,
            other.template_name,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.workload_name,
            other.workload_name,
        );
    }
}
