#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SleepActionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds: Option<i64>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::core::v1::SleepAction {
    type Optioned = SleepActionAc;
}
#[automatically_derived]
impl crate::Optionable for SleepActionAc {
    type Optioned = SleepActionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::core::v1::SleepAction {
    fn into_optioned(self) -> SleepActionAc {
        SleepActionAc {
            seconds: Some(self.seconds),
        }
    }
    fn try_from_optioned(value: SleepActionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            seconds: value
                .seconds
                .ok_or(crate::Error {
                    missing_field: "seconds",
                })?,
        })
    }
    fn merge(&mut self, other: SleepActionAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.seconds {
            self.seconds = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::core::v1::SleepAction>
for SleepActionAc {
    fn from_optionable(value: k8s_openapi026::api::core::v1::SleepAction) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::core::v1::SleepAction, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::core::v1::SleepAction,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
