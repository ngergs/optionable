pub struct PodReadinessGateOpt {
    pub condition_type: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::pod_readiness_gate::PodReadinessGate {
    type Optioned = PodReadinessGateOpt;
}
#[automatically_derived]
impl crate::Optionable for PodReadinessGateOpt {
    type Optioned = PodReadinessGateOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::pod_readiness_gate::PodReadinessGate {
    fn into_optioned(self) -> PodReadinessGateOpt {
        PodReadinessGateOpt {
            condition_type: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.condition_type,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: PodReadinessGateOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            condition_type: <std::string::String as crate::OptionableConvert>::try_from_optioned(
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
        other: PodReadinessGateOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.condition_type {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.condition_type,
                other_value,
            )?;
        }
        Ok(())
    }
}
