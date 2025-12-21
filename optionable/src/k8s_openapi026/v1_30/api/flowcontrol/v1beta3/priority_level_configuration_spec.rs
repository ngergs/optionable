#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PriorityLevelConfigurationSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt: <Option<
        ::k8s_openapi026::api::flowcontrol::v1beta3::ExemptPriorityLevelConfiguration,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited: <Option<
        ::k8s_openapi026::api::flowcontrol::v1beta3::LimitedPriorityLevelConfiguration,
    > as crate::Optionable>::Optioned,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::api::flowcontrol::v1beta3::PriorityLevelConfigurationSpec {
    type Optioned = PriorityLevelConfigurationSpecAc;
}
#[automatically_derived]
impl crate::Optionable for PriorityLevelConfigurationSpecAc {
    type Optioned = PriorityLevelConfigurationSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::flowcontrol::v1beta3::PriorityLevelConfigurationSpec {
    fn into_optioned(self) -> PriorityLevelConfigurationSpecAc {
        PriorityLevelConfigurationSpecAc {
            exempt: crate::OptionableConvert::into_optioned(self.exempt),
            limited: crate::OptionableConvert::into_optioned(self.limited),
            type_: Some(crate::OptionableConvert::into_optioned(self.type_)),
        }
    }
    fn try_from_optioned(
        value: PriorityLevelConfigurationSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            exempt: crate::OptionableConvert::try_from_optioned(value.exempt)?,
            limited: crate::OptionableConvert::try_from_optioned(value.limited)?,
            type_: crate::OptionableConvert::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::Error {
                        missing_field: "type_",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PriorityLevelConfigurationSpecAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.exempt, other.exempt)?;
        crate::OptionableConvert::merge(&mut self.limited, other.limited)?;
        if let Some(other_value) = other.type_ {
            crate::OptionableConvert::merge(&mut self.type_, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::api::flowcontrol::v1beta3::PriorityLevelConfigurationSpec,
> for PriorityLevelConfigurationSpecAc {
    fn from_optionable(
        value: k8s_openapi026::api::flowcontrol::v1beta3::PriorityLevelConfigurationSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::flowcontrol::v1beta3::PriorityLevelConfigurationSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::flowcontrol::v1beta3::PriorityLevelConfigurationSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
