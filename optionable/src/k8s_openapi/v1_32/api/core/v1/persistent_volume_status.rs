#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PersistentVolumeStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_phase_transition_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PersistentVolumeStatus {
    type Optioned = PersistentVolumeStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PersistentVolumeStatusAc {
    type Optioned = PersistentVolumeStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PersistentVolumeStatus {
    fn into_optioned(self) -> PersistentVolumeStatusAc {
        PersistentVolumeStatusAc {
            last_phase_transition_time: crate::OptionableConvert::into_optioned(
                self.last_phase_transition_time,
            ),
            message: crate::OptionableConvert::into_optioned(self.message),
            phase: crate::OptionableConvert::into_optioned(self.phase),
            reason: crate::OptionableConvert::into_optioned(self.reason),
        }
    }
    fn try_from_optioned(value: PersistentVolumeStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            last_phase_transition_time: crate::OptionableConvert::try_from_optioned(
                value.last_phase_transition_time,
            )?,
            message: crate::OptionableConvert::try_from_optioned(value.message)?,
            phase: crate::OptionableConvert::try_from_optioned(value.phase)?,
            reason: crate::OptionableConvert::try_from_optioned(value.reason)?,
        })
    }
    fn merge(&mut self, other: PersistentVolumeStatusAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.last_phase_transition_time,
            other.last_phase_transition_time,
        )?;
        crate::OptionableConvert::merge(&mut self.message, other.message)?;
        crate::OptionableConvert::merge(&mut self.phase, other.phase)?;
        crate::OptionableConvert::merge(&mut self.reason, other.reason)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::PersistentVolumeStatus>
for PersistentVolumeStatusAc {
    fn from_optionable(
        value: ::k8s_openapi::api::core::v1::PersistentVolumeStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::PersistentVolumeStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::PersistentVolumeStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
