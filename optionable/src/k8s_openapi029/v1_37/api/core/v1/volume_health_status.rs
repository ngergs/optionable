#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// VolumeHealthStatus contains health information for a volume reported by the CSI controller plugin.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VolumeHealthStatusAc {
    /// conditions is the set of adverse conditions reported by the CSI controller plugin. An empty list means no adverse condition. At most 16 conditions may be reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_conditions: Option<
        std::vec::Vec<
            <::k8s_openapi029::api::core::v1::VolumeHealthCondition as crate::Optionable>::Optioned,
        >,
    >,
    /// lastTransitionTime is when the current set of conditions first appeared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<
        <::k8s_openapi029::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::core::v1::VolumeHealthStatus {
    type Optioned = VolumeHealthStatusAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeHealthStatusAc {
    type Optioned = VolumeHealthStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi029::api::core::v1::VolumeHealthStatus {
    fn into_optioned(self) -> VolumeHealthStatusAc {
        VolumeHealthStatusAc {
            health_conditions: crate::OptionableConvert::into_optioned(
                self.health_conditions,
            ),
            last_transition_time: crate::OptionableConvert::into_optioned(
                self.last_transition_time,
            ),
        }
    }
    fn try_from_optioned(value: VolumeHealthStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            health_conditions: crate::OptionableConvert::try_from_optioned(
                value.health_conditions,
            )?,
            last_transition_time: crate::OptionableConvert::try_from_optioned(
                value.last_transition_time,
            )?,
        })
    }
    fn merge(&mut self, other: VolumeHealthStatusAc) -> Result<(), crate::Error> {
        if self.health_conditions.is_none() {
            self.health_conditions = crate::OptionableConvert::try_from_optioned(
                other.health_conditions,
            )?;
        } else if let Some(self_value) = self.health_conditions.as_mut()
            && let Some(other_value) = other.health_conditions
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if self.last_transition_time.is_none() {
            self.last_transition_time = crate::OptionableConvert::try_from_optioned(
                other.last_transition_time,
            )?;
        } else if let Some(self_value) = self.last_transition_time.as_mut()
            && let Some(other_value) = other.last_transition_time
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::core::v1::VolumeHealthStatus>
for VolumeHealthStatusAc {
    fn from_optionable(
        value: k8s_openapi029::api::core::v1::VolumeHealthStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi029::api::core::v1::VolumeHealthStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::core::v1::VolumeHealthStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for VolumeHealthStatusAc {
    fn merge_from(&mut self, other: Self) {
        crate::k8s_openapi::merge::merge_map_option_wrapped(
            &mut self.health_conditions,
            other.health_conditions,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.last_transition_time,
            other.last_transition_time,
        );
    }
}
