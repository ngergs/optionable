pub struct PodAntiAffinityOpt {
    pub preferred_during_scheduling_ignored_during_execution: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::WeightedPodAffinityTerm>,
    > as crate::Optionable>::Optioned,
    pub required_during_scheduling_ignored_during_execution: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::PodAffinityTerm>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::pod_anti_affinity::PodAntiAffinity {
    type Optioned = PodAntiAffinityOpt;
}
#[automatically_derived]
impl crate::Optionable for PodAntiAffinityOpt {
    type Optioned = PodAntiAffinityOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::pod_anti_affinity::PodAntiAffinity {
    fn into_optioned(self) -> PodAntiAffinityOpt {
        PodAntiAffinityOpt {
            preferred_during_scheduling_ignored_during_execution: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::WeightedPodAffinityTerm>,
            > as crate::OptionableConvert>::into_optioned(
                self.preferred_during_scheduling_ignored_during_execution,
            ),
            required_during_scheduling_ignored_during_execution: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::PodAffinityTerm>,
            > as crate::OptionableConvert>::into_optioned(
                self.required_during_scheduling_ignored_during_execution,
            ),
        }
    }
    fn try_from_optioned(
        value: PodAntiAffinityOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            preferred_during_scheduling_ignored_during_execution: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::WeightedPodAffinityTerm>,
            > as crate::OptionableConvert>::try_from_optioned(
                value.preferred_during_scheduling_ignored_during_execution,
            )?,
            required_during_scheduling_ignored_during_execution: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::PodAffinityTerm>,
            > as crate::OptionableConvert>::try_from_optioned(
                value.required_during_scheduling_ignored_during_execution,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PodAntiAffinityOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::WeightedPodAffinityTerm>,
        > as crate::OptionableConvert>::merge(
            &mut self.preferred_during_scheduling_ignored_during_execution,
            other.preferred_during_scheduling_ignored_during_execution,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::PodAffinityTerm>,
        > as crate::OptionableConvert>::merge(
            &mut self.required_during_scheduling_ignored_during_execution,
            other.required_during_scheduling_ignored_during_execution,
        )?;
        Ok(())
    }
}
