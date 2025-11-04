#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct PodReadinessGateAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PodReadinessGate {
    type Optioned = PodReadinessGateAc;
}
#[automatically_derived]
impl crate::Optionable for PodReadinessGateAc {
    type Optioned = PodReadinessGateAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PodReadinessGate {
    fn into_optioned(self) -> PodReadinessGateAc {
        PodReadinessGateAc {
            condition_type: Some(
                crate::OptionableConvert::into_optioned(self.condition_type),
            ),
        }
    }
    fn try_from_optioned(
        value: PodReadinessGateAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            condition_type: crate::OptionableConvert::try_from_optioned(
                value
                    .condition_type
                    .ok_or(crate::optionable::Error {
                        missing_field: "condition_type",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PodReadinessGateAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.condition_type {
            crate::OptionableConvert::merge(&mut self.condition_type, other_value)?;
        }
        Ok(())
    }
}
