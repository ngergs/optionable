#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// WorkloadPodGroupDisruptionMode defines how individual pods within a group can be disrupted. Exactly one mode must be set.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkloadPodGroupDisruptionModeAc {
    /// all specifies that all pods in the group must be disrupted together.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<
        <::k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupAllDisruptionMode as crate::Optionable>::Optioned,
    >,
    /// single specifies that pods can be disrupted independently from each other.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single: Option<
        <::k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupSingleDisruptionMode as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupDisruptionMode {
    type Optioned = WorkloadPodGroupDisruptionModeAc;
}
#[automatically_derived]
impl crate::Optionable for WorkloadPodGroupDisruptionModeAc {
    type Optioned = WorkloadPodGroupDisruptionModeAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupDisruptionMode {
    fn into_optioned(self) -> WorkloadPodGroupDisruptionModeAc {
        WorkloadPodGroupDisruptionModeAc {
            all: crate::OptionableConvert::into_optioned(self.all),
            single: crate::OptionableConvert::into_optioned(self.single),
        }
    }
    fn try_from_optioned(
        value: WorkloadPodGroupDisruptionModeAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            all: crate::OptionableConvert::try_from_optioned(value.all)?,
            single: crate::OptionableConvert::try_from_optioned(value.single)?,
        })
    }
    fn merge(
        &mut self,
        other: WorkloadPodGroupDisruptionModeAc,
    ) -> Result<(), crate::Error> {
        if self.all.is_none() {
            self.all = crate::OptionableConvert::try_from_optioned(other.all)?;
        } else if let Some(self_value) = self.all.as_mut()
            && let Some(other_value) = other.all
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.single.is_none() {
            self.single = crate::OptionableConvert::try_from_optioned(other.single)?;
        } else if let Some(self_value) = self.single.as_mut()
            && let Some(other_value) = other.single
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupDisruptionMode,
> for WorkloadPodGroupDisruptionModeAc {
    fn from_optionable(
        value: k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupDisruptionMode,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupDisruptionMode,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupDisruptionMode,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for WorkloadPodGroupDisruptionModeAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.all, other.all);
        k8s_openapi029::DeepMerge::merge_from(&mut self.single, other.single);
    }
}
