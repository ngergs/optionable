#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodAntiAffinityAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_during_scheduling_ignored_during_execution: <Option<
        std::vec::Vec<::k8s_openapi027::api::core::v1::WeightedPodAffinityTerm>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_during_scheduling_ignored_during_execution: <Option<
        std::vec::Vec<::k8s_openapi027::api::core::v1::PodAffinityTerm>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PodAntiAffinity {
    type Optioned = PodAntiAffinityAc;
}
#[automatically_derived]
impl crate::Optionable for PodAntiAffinityAc {
    type Optioned = PodAntiAffinityAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::PodAntiAffinity {
    fn into_optioned(self) -> PodAntiAffinityAc {
        PodAntiAffinityAc {
            preferred_during_scheduling_ignored_during_execution: crate::OptionableConvert::into_optioned(
                self.preferred_during_scheduling_ignored_during_execution,
            ),
            required_during_scheduling_ignored_during_execution: crate::OptionableConvert::into_optioned(
                self.required_during_scheduling_ignored_during_execution,
            ),
        }
    }
    fn try_from_optioned(value: PodAntiAffinityAc) -> Result<Self, crate::Error> {
        Ok(Self {
            preferred_during_scheduling_ignored_during_execution: crate::OptionableConvert::try_from_optioned(
                value.preferred_during_scheduling_ignored_during_execution,
            )?,
            required_during_scheduling_ignored_during_execution: crate::OptionableConvert::try_from_optioned(
                value.required_during_scheduling_ignored_during_execution,
            )?,
        })
    }
    fn merge(&mut self, other: PodAntiAffinityAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.preferred_during_scheduling_ignored_during_execution,
            other.preferred_during_scheduling_ignored_during_execution,
        )?;
        crate::OptionableConvert::merge(
            &mut self.required_during_scheduling_ignored_during_execution,
            other.required_during_scheduling_ignored_during_execution,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PodAntiAffinity>
for PodAntiAffinityAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::PodAntiAffinity) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PodAntiAffinity, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PodAntiAffinity,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
