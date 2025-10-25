pub struct HPAScalingPolicyOpt {
    pub period_seconds: Option<i32>,
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub value: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::autoscaling::v2::HPAScalingPolicy {
    type Optioned = HPAScalingPolicyOpt;
}
#[automatically_derived]
impl crate::Optionable for HPAScalingPolicyOpt {
    type Optioned = HPAScalingPolicyOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::autoscaling::v2::HPAScalingPolicy {
    fn into_optioned(self) -> HPAScalingPolicyOpt {
        HPAScalingPolicyOpt {
            period_seconds: Some(self.period_seconds),
            type_: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.type_,
                ),
            ),
            value: Some(self.value),
        }
    }
    fn try_from_optioned(
        value: HPAScalingPolicyOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            period_seconds: value
                .period_seconds
                .ok_or(crate::optionable::Error {
                    missing_field: "period_seconds",
                })?,
            type_: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::optionable::Error {
                        missing_field: "type_",
                    })?,
            )?,
            value: value
                .value
                .ok_or(crate::optionable::Error {
                    missing_field: "value",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: HPAScalingPolicyOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.period_seconds {
            self.period_seconds = other_value;
        }
        if let Some(other_value) = other.type_ {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.type_,
                other_value,
            )?;
        }
        if let Some(other_value) = other.value {
            self.value = other_value;
        }
        Ok(())
    }
}
