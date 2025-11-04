#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
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
    fn try_from_optioned(
        value: PersistentVolumeStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            last_phase_transition_time: crate::OptionableConvert::try_from_optioned(
                value.last_phase_transition_time,
            )?,
            message: crate::OptionableConvert::try_from_optioned(value.message)?,
            phase: crate::OptionableConvert::try_from_optioned(value.phase)?,
            reason: crate::OptionableConvert::try_from_optioned(value.reason)?,
        })
    }
    fn merge(
        &mut self,
        other: PersistentVolumeStatusAc,
    ) -> Result<(), crate::optionable::Error> {
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
