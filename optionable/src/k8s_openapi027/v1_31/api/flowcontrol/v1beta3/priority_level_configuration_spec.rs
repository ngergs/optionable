#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PriorityLevelConfigurationSpec specifies the configuration of a priority level.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PriorityLevelConfigurationSpecAc {
    /// `exempt` specifies how requests are handled for an exempt priority level. This field MUST be empty if `type` is `"Limited"`. This field MAY be non-empty if `type` is `"Exempt"`. If empty and `type` is `"Exempt"` then the default values for `ExemptPriorityLevelConfiguration` apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt: Option<
        <::k8s_openapi027::api::flowcontrol::v1beta3::ExemptPriorityLevelConfiguration as crate::Optionable>::Optioned,
    >,
    /// `limited` specifies how requests are handled for a Limited priority level. This field must be non-empty if and only if `type` is `"Limited"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited: Option<
        <::k8s_openapi027::api::flowcontrol::v1beta3::LimitedPriorityLevelConfiguration as crate::Optionable>::Optioned,
    >,
    /// `type` indicates whether this priority level is subject to limitation on request execution.  A value of `"Exempt"` means that requests of this priority level are not subject to a limit (and thus are never queued) and do not detract from the capacity made available to other priority levels.  A value of `"Limited"` means that (a) requests of this priority level _are_ subject to limits and (b) some of the server's limited capacity is made available exclusively to this priority level. Required.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::flowcontrol::v1beta3::PriorityLevelConfigurationSpec {
    type Optioned = PriorityLevelConfigurationSpecAc;
}
#[automatically_derived]
impl crate::Optionable for PriorityLevelConfigurationSpecAc {
    type Optioned = PriorityLevelConfigurationSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::flowcontrol::v1beta3::PriorityLevelConfigurationSpec {
    fn into_optioned(self) -> PriorityLevelConfigurationSpecAc {
        PriorityLevelConfigurationSpecAc {
            exempt: crate::OptionableConvert::into_optioned(self.exempt),
            limited: crate::OptionableConvert::into_optioned(self.limited),
            type_: Some(self.type_),
        }
    }
    fn try_from_optioned(
        value: PriorityLevelConfigurationSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            exempt: crate::OptionableConvert::try_from_optioned(value.exempt)?,
            limited: crate::OptionableConvert::try_from_optioned(value.limited)?,
            type_: value
                .type_
                .ok_or(crate::Error {
                    missing_field: "type_",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: PriorityLevelConfigurationSpecAc,
    ) -> Result<(), crate::Error> {
        if self.exempt.is_none() {
            self.exempt = crate::OptionableConvert::try_from_optioned(other.exempt)?;
        } else if let Some(self_value) = self.exempt.as_mut()
            && let Some(other_value) = other.exempt
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.limited.is_none() {
            self.limited = crate::OptionableConvert::try_from_optioned(other.limited)?;
        } else if let Some(self_value) = self.limited.as_mut()
            && let Some(other_value) = other.limited
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.type_ {
            self.type_ = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::flowcontrol::v1beta3::PriorityLevelConfigurationSpec,
> for PriorityLevelConfigurationSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::flowcontrol::v1beta3::PriorityLevelConfigurationSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::flowcontrol::v1beta3::PriorityLevelConfigurationSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::flowcontrol::v1beta3::PriorityLevelConfigurationSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
