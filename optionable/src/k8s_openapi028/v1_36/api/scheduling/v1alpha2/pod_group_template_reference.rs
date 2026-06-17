#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodGroupTemplateReference references a PodGroup template defined in some object (e.g. Workload). Exactly one reference must be set.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodGroupTemplateReferenceAc {
    /// Workload references the PodGroupTemplate within the Workload object that was used to create the PodGroup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload: Option<
        <::k8s_openapi028::api::scheduling::v1alpha2::WorkloadPodGroupTemplateReference as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi028::api::scheduling::v1alpha2::PodGroupTemplateReference {
    type Optioned = PodGroupTemplateReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for PodGroupTemplateReferenceAc {
    type Optioned = PodGroupTemplateReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi028::api::scheduling::v1alpha2::PodGroupTemplateReference {
    fn into_optioned(self) -> PodGroupTemplateReferenceAc {
        PodGroupTemplateReferenceAc {
            workload: crate::OptionableConvert::into_optioned(self.workload),
        }
    }
    fn try_from_optioned(
        value: PodGroupTemplateReferenceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            workload: crate::OptionableConvert::try_from_optioned(value.workload)?,
        })
    }
    fn merge(&mut self, other: PodGroupTemplateReferenceAc) -> Result<(), crate::Error> {
        if self.workload.is_none() {
            self.workload = crate::OptionableConvert::try_from_optioned(other.workload)?;
        } else if let Some(self_value) = self.workload.as_mut()
            && let Some(other_value) = other.workload
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi028::api::scheduling::v1alpha2::PodGroupTemplateReference,
> for PodGroupTemplateReferenceAc {
    fn from_optionable(
        value: k8s_openapi028::api::scheduling::v1alpha2::PodGroupTemplateReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi028::api::scheduling::v1alpha2::PodGroupTemplateReference,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::api::scheduling::v1alpha2::PodGroupTemplateReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::DeepMerge for PodGroupTemplateReferenceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi028::DeepMerge::merge_from(&mut self.workload, other.workload);
    }
}
