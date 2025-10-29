pub struct PriorityLevelConfigurationReferenceOpt {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::flowcontrol::v1::PriorityLevelConfigurationReference {
    type Optioned = PriorityLevelConfigurationReferenceOpt;
}
#[automatically_derived]
impl crate::Optionable for PriorityLevelConfigurationReferenceOpt {
    type Optioned = PriorityLevelConfigurationReferenceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::PriorityLevelConfigurationReference {
    fn into_optioned(self) -> PriorityLevelConfigurationReferenceOpt {
        PriorityLevelConfigurationReferenceOpt {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(
        value: PriorityLevelConfigurationReferenceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PriorityLevelConfigurationReferenceOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}
