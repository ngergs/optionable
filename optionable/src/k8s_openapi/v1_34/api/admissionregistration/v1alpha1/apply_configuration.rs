pub struct ApplyConfigurationOpt {
    pub expression: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1alpha1::apply_configuration::ApplyConfiguration {
    type Optioned = ApplyConfigurationOpt;
}
#[automatically_derived]
impl crate::Optionable for ApplyConfigurationOpt {
    type Optioned = ApplyConfigurationOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1alpha1::apply_configuration::ApplyConfiguration {
    fn into_optioned(self) -> ApplyConfigurationOpt {
        ApplyConfigurationOpt {
            expression: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.expression),
        }
    }
    fn try_from_optioned(
        value: ApplyConfigurationOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            expression: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.expression)?,
        })
    }
    fn merge(
        &mut self,
        other: ApplyConfigurationOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.expression, other.expression)?;
        Ok(())
    }
}
