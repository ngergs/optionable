#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PersistentVolumeStatus is the current status of a persistent volume.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PersistentVolumeStatusAc {
    /// lastPhaseTransitionTime is the time the phase transitioned from one to another and automatically resets to current time everytime a volume phase transitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_phase_transition_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// message is a human-readable message indicating details about why the volume is in this state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// phase indicates if a volume is available, bound to a claim, or released by a claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#phase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<std::string::String>,
    /// reason is a brief CamelCase string that describes any failure and is meant for machine parsing and tidy display in the CLI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PersistentVolumeStatus {
    type Optioned = PersistentVolumeStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PersistentVolumeStatusAc {
    type Optioned = PersistentVolumeStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::PersistentVolumeStatus {
    fn into_optioned(self) -> PersistentVolumeStatusAc {
        PersistentVolumeStatusAc {
            last_phase_transition_time: crate::OptionableConvert::into_optioned(
                self.last_phase_transition_time,
            ),
            message: self.message,
            phase: self.phase,
            reason: self.reason,
        }
    }
    fn try_from_optioned(value: PersistentVolumeStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            last_phase_transition_time: crate::OptionableConvert::try_from_optioned(
                value.last_phase_transition_time,
            )?,
            message: value.message,
            phase: value.phase,
            reason: value.reason,
        })
    }
    fn merge(&mut self, other: PersistentVolumeStatusAc) -> Result<(), crate::Error> {
        if self.last_phase_transition_time.is_none() {
            self.last_phase_transition_time = crate::OptionableConvert::try_from_optioned(
                other.last_phase_transition_time,
            )?;
        } else if let Some(self_value) = self.last_phase_transition_time.as_mut()
            && let Some(other_value) = other.last_phase_transition_time
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.message.is_none() {
            self.message = crate::OptionableConvert::try_from_optioned(other.message)?;
        } else if let Some(self_value) = self.message.as_mut()
            && let Some(other_value) = other.message
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.phase.is_none() {
            self.phase = crate::OptionableConvert::try_from_optioned(other.phase)?;
        } else if let Some(self_value) = self.phase.as_mut()
            && let Some(other_value) = other.phase
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.reason.is_none() {
            self.reason = crate::OptionableConvert::try_from_optioned(other.reason)?;
        } else if let Some(self_value) = self.reason.as_mut()
            && let Some(other_value) = other.reason
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PersistentVolumeStatus>
for PersistentVolumeStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::PersistentVolumeStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PersistentVolumeStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PersistentVolumeStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for PersistentVolumeStatusAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.last_phase_transition_time,
            other.last_phase_transition_time,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.message, other.message);
        k8s_openapi027::DeepMerge::merge_from(&mut self.phase, other.phase);
        k8s_openapi027::DeepMerge::merge_from(&mut self.reason, other.reason);
    }
}
