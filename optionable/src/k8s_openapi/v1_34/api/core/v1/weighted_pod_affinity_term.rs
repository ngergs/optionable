pub struct WeightedPodAffinityTermOpt {
    pub pod_affinity_term: Option<
        <::k8s_openapi::api::core::v1::PodAffinityTerm as crate::Optionable>::Optioned,
    >,
    pub weight: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::WeightedPodAffinityTerm {
    type Optioned = WeightedPodAffinityTermOpt;
}
#[automatically_derived]
impl crate::Optionable for WeightedPodAffinityTermOpt {
    type Optioned = WeightedPodAffinityTermOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::WeightedPodAffinityTerm {
    fn into_optioned(self) -> WeightedPodAffinityTermOpt {
        WeightedPodAffinityTermOpt {
            pod_affinity_term: Some(
                crate::OptionableConvert::into_optioned(self.pod_affinity_term),
            ),
            weight: Some(self.weight),
        }
    }
    fn try_from_optioned(
        value: WeightedPodAffinityTermOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            pod_affinity_term: crate::OptionableConvert::try_from_optioned(
                value
                    .pod_affinity_term
                    .ok_or(crate::optionable::Error {
                        missing_field: "pod_affinity_term",
                    })?,
            )?,
            weight: value
                .weight
                .ok_or(crate::optionable::Error {
                    missing_field: "weight",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: WeightedPodAffinityTermOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.pod_affinity_term {
            crate::OptionableConvert::merge(&mut self.pod_affinity_term, other_value)?;
        }
        if let Some(other_value) = other.weight {
            self.weight = other_value;
        }
        Ok(())
    }
}
