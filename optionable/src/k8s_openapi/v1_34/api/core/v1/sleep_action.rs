#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SleepActionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds: Option<i64>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::SleepAction {
    type Optioned = SleepActionAc;
}
#[automatically_derived]
impl crate::Optionable for SleepActionAc {
    type Optioned = SleepActionAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::SleepAction {
    fn into_optioned(self) -> SleepActionAc {
        SleepActionAc {
            seconds: Some(self.seconds),
        }
    }
    fn try_from_optioned(
        value: SleepActionAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            seconds: value
                .seconds
                .ok_or(crate::optionable::Error {
                    missing_field: "seconds",
                })?,
        })
    }
    fn merge(&mut self, other: SleepActionAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.seconds {
            self.seconds = other_value;
        }
        Ok(())
    }
}
