pub struct PodFailurePolicyOnPodConditionsPatternOpt {
    pub status: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::batch::v1::PodFailurePolicyOnPodConditionsPattern {
    type Optioned = PodFailurePolicyOnPodConditionsPatternOpt;
}
#[automatically_derived]
impl crate::Optionable for PodFailurePolicyOnPodConditionsPatternOpt {
    type Optioned = PodFailurePolicyOnPodConditionsPatternOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::batch::v1::PodFailurePolicyOnPodConditionsPattern {
    fn into_optioned(self) -> PodFailurePolicyOnPodConditionsPatternOpt {
        PodFailurePolicyOnPodConditionsPatternOpt {
            status: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.status,
                ),
            ),
            type_: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.type_,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: PodFailurePolicyOnPodConditionsPatternOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            status: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .status
                    .ok_or(crate::optionable::Error {
                        missing_field: "status",
                    })?,
            )?,
            type_: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::optionable::Error {
                        missing_field: "type_",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PodFailurePolicyOnPodConditionsPatternOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.status {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.status,
                other_value,
            )?;
        }
        if let Some(other_value) = other.type_ {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.type_,
                other_value,
            )?;
        }
        Ok(())
    }
}
