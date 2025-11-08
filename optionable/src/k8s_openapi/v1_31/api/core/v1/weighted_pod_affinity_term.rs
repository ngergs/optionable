#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct WeightedPodAffinityTermAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_affinity_term: Option<
        <::k8s_openapi::api::core::v1::PodAffinityTerm as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::WeightedPodAffinityTerm {
    type Optioned = WeightedPodAffinityTermAc;
}
#[automatically_derived]
impl crate::Optionable for WeightedPodAffinityTermAc {
    type Optioned = WeightedPodAffinityTermAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::WeightedPodAffinityTerm {
    fn into_optioned(self) -> WeightedPodAffinityTermAc {
        WeightedPodAffinityTermAc {
            pod_affinity_term: Some(
                crate::OptionableConvert::into_optioned(self.pod_affinity_term),
            ),
            weight: Some(self.weight),
        }
    }
    fn try_from_optioned(
        value: WeightedPodAffinityTermAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            pod_affinity_term: crate::OptionableConvert::try_from_optioned(
                value
                    .pod_affinity_term
                    .ok_or(crate::Error {
                        missing_field: "pod_affinity_term",
                    })?,
            )?,
            weight: value
                .weight
                .ok_or(crate::Error {
                    missing_field: "weight",
                })?,
        })
    }
    fn merge(&mut self, other: WeightedPodAffinityTermAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.pod_affinity_term {
            crate::OptionableConvert::merge(&mut self.pod_affinity_term, other_value)?;
        }
        if let Some(other_value) = other.weight {
            self.weight = other_value;
        }
        Ok(())
    }
}
