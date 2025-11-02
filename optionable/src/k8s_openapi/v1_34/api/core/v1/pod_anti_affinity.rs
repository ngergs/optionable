#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PodAntiAffinityAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_during_scheduling_ignored_during_execution: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::WeightedPodAffinityTerm>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_during_scheduling_ignored_during_execution: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::PodAffinityTerm>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PodAntiAffinity {
    type Optioned = PodAntiAffinityAc;
}
#[automatically_derived]
impl crate::Optionable for PodAntiAffinityAc {
    type Optioned = PodAntiAffinityAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PodAntiAffinity {
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
    fn try_from_optioned(
        value: PodAntiAffinityAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            preferred_during_scheduling_ignored_during_execution: crate::OptionableConvert::try_from_optioned(
                value.preferred_during_scheduling_ignored_during_execution,
            )?,
            required_during_scheduling_ignored_during_execution: crate::OptionableConvert::try_from_optioned(
                value.required_during_scheduling_ignored_during_execution,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PodAntiAffinityAc,
    ) -> Result<(), crate::optionable::Error> {
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
