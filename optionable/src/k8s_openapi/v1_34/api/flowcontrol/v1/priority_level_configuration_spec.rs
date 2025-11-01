pub struct PriorityLevelConfigurationSpecAc {
    pub exempt: <Option<
        ::k8s_openapi::api::flowcontrol::v1::ExemptPriorityLevelConfiguration,
    > as crate::Optionable>::Optioned,
    pub limited: <Option<
        ::k8s_openapi::api::flowcontrol::v1::LimitedPriorityLevelConfiguration,
    > as crate::Optionable>::Optioned,
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
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            exempt: crate::OptionableConvert::try_from_optioned(value.exempt)?,
            limited: crate::OptionableConvert::try_from_optioned(value.limited)?,
            type_: crate::OptionableConvert::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::optionable::Error {
                        missing_field: "type_",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PriorityLevelConfigurationSpecAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.exempt, other.exempt)?;
        crate::OptionableConvert::merge(&mut self.limited, other.limited)?;
        if let Some(other_value) = other.type_ {
            crate::OptionableConvert::merge(&mut self.type_, other_value)?;
        }
        Ok(())
    }
}
