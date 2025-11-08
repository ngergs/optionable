#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct PriorityLevelConfigurationSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt: <Option<
        ::k8s_openapi::api::flowcontrol::v1::ExemptPriorityLevelConfiguration,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited: <Option<
        ::k8s_openapi::api::flowcontrol::v1::LimitedPriorityLevelConfiguration,
    > as crate::Optionable>::Optioned,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::flowcontrol::v1::PriorityLevelConfigurationSpec {
    type Optioned = PriorityLevelConfigurationSpecAc;
}
#[automatically_derived]
impl crate::Optionable for PriorityLevelConfigurationSpecAc {
    type Optioned = PriorityLevelConfigurationSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::PriorityLevelConfigurationSpec {
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
