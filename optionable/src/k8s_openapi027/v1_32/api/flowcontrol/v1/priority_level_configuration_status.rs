#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PriorityLevelConfigurationStatus represents the current state of a "request-priority".
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PriorityLevelConfigurationStatusAc {
    /// `conditions` is the current state of "request-priority".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::flowcontrol::v1::PriorityLevelConfigurationCondition as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::flowcontrol::v1::PriorityLevelConfigurationStatus {
    type Optioned = PriorityLevelConfigurationStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PriorityLevelConfigurationStatusAc {
    type Optioned = PriorityLevelConfigurationStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::flowcontrol::v1::PriorityLevelConfigurationStatus {
    fn into_optioned(self) -> PriorityLevelConfigurationStatusAc {
        PriorityLevelConfigurationStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
        }
    }
    fn try_from_optioned(
        value: PriorityLevelConfigurationStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
        })
    }
    fn merge(
        &mut self,
        other: PriorityLevelConfigurationStatusAc,
    ) -> Result<(), crate::Error> {
        if self.conditions.is_none() {
            self.conditions = crate::OptionableConvert::try_from_optioned(
                other.conditions,
            )?;
        } else if let Some(self_value) = self.conditions.as_mut()
            && let Some(other_value) = other.conditions
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::flowcontrol::v1::PriorityLevelConfigurationStatus,
> for PriorityLevelConfigurationStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::flowcontrol::v1::PriorityLevelConfigurationStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::flowcontrol::v1::PriorityLevelConfigurationStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::flowcontrol::v1::PriorityLevelConfigurationStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for PriorityLevelConfigurationStatusAc {
    fn merge_from(&mut self, other: Self) {
        crate::k8s_openapi::merge::merge_map(&mut self.conditions, other.conditions);
    }
}
