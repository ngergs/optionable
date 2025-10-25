pub struct PodSchedulingGateOpt {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PodSchedulingGate {
    type Optioned = PodSchedulingGateOpt;
}
#[automatically_derived]
impl crate::Optionable for PodSchedulingGateOpt {
    type Optioned = PodSchedulingGateOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PodSchedulingGate {
    fn into_optioned(self) -> PodSchedulingGateOpt {
        PodSchedulingGateOpt {
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: PodSchedulingGateOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PodSchedulingGateOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        Ok(())
    }
}
