pub struct SleepActionOpt {
    pub seconds: Option<i64>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::SleepAction {
    type Optioned = SleepActionOpt;
}
#[automatically_derived]
impl crate::Optionable for SleepActionOpt {
    type Optioned = SleepActionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::SleepAction {
    fn into_optioned(self) -> SleepActionOpt {
        SleepActionOpt {
            seconds: Some(self.seconds),
        }
    }
    fn try_from_optioned(
        value: SleepActionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            seconds: value
                .seconds
                .ok_or(crate::optionable::Error {
                    missing_field: "seconds",
                })?,
        })
    }
    fn merge(&mut self, other: SleepActionOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.seconds {
            self.seconds = other_value;
        }
        Ok(())
    }
}
