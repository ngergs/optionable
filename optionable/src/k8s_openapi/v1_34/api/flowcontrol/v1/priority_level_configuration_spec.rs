pub struct PriorityLevelConfigurationSpecOpt {
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
for ::k8s_openapi::api::flowcontrol::v1::priority_level_configuration_spec::PriorityLevelConfigurationSpec {
    type Optioned = PriorityLevelConfigurationSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for PriorityLevelConfigurationSpecOpt {
    type Optioned = PriorityLevelConfigurationSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::priority_level_configuration_spec::PriorityLevelConfigurationSpec {
    fn into_optioned(self) -> PriorityLevelConfigurationSpecOpt {
        PriorityLevelConfigurationSpecOpt {
            exempt: <Option<
                ::k8s_openapi::api::flowcontrol::v1::ExemptPriorityLevelConfiguration,
            > as crate::OptionableConvert>::into_optioned(self.exempt),
            limited: <Option<
                ::k8s_openapi::api::flowcontrol::v1::LimitedPriorityLevelConfiguration,
            > as crate::OptionableConvert>::into_optioned(self.limited),
            type_: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.type_,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: PriorityLevelConfigurationSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            exempt: <Option<
                ::k8s_openapi::api::flowcontrol::v1::ExemptPriorityLevelConfiguration,
            > as crate::OptionableConvert>::try_from_optioned(value.exempt)?,
            limited: <Option<
                ::k8s_openapi::api::flowcontrol::v1::LimitedPriorityLevelConfiguration,
            > as crate::OptionableConvert>::try_from_optioned(value.limited)?,
            type_: <std::string::String as crate::OptionableConvert>::try_from_optioned(
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
        other: PriorityLevelConfigurationSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::flowcontrol::v1::ExemptPriorityLevelConfiguration,
        > as crate::OptionableConvert>::merge(&mut self.exempt, other.exempt)?;
        <Option<
            ::k8s_openapi::api::flowcontrol::v1::LimitedPriorityLevelConfiguration,
        > as crate::OptionableConvert>::merge(&mut self.limited, other.limited)?;
        if let Some(other_value) = other.type_ {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.type_,
                other_value,
            )?;
        }
        Ok(())
    }
}
