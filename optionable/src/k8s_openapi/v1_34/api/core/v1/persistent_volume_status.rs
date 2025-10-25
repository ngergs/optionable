pub struct PersistentVolumeStatusOpt {
    pub last_phase_transition_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub phase: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::persistent_volume_status::PersistentVolumeStatus {
    type Optioned = PersistentVolumeStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for PersistentVolumeStatusOpt {
    type Optioned = PersistentVolumeStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::persistent_volume_status::PersistentVolumeStatus {
    fn into_optioned(self) -> PersistentVolumeStatusOpt {
        PersistentVolumeStatusOpt {
            last_phase_transition_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::into_optioned(
                self.last_phase_transition_time,
            ),
            message: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.message),
            phase: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.phase),
            reason: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.reason),
        }
    }
    fn try_from_optioned(
        value: PersistentVolumeStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            last_phase_transition_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::try_from_optioned(
                value.last_phase_transition_time,
            )?,
            message: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.message)?,
            phase: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.phase)?,
            reason: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.reason)?,
        })
    }
    fn merge(
        &mut self,
        other: PersistentVolumeStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
        > as crate::OptionableConvert>::merge(
            &mut self.last_phase_transition_time,
            other.last_phase_transition_time,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.message, other.message)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.phase, other.phase)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.reason, other.reason)?;
        Ok(())
    }
}
