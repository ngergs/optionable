pub struct VariableOpt {
    pub expression: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1alpha1::Variable {
    type Optioned = VariableOpt;
}
#[automatically_derived]
impl crate::Optionable for VariableOpt {
    type Optioned = VariableOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1alpha1::Variable {
    fn into_optioned(self) -> VariableOpt {
        VariableOpt {
            expression: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.expression,
                ),
            ),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
        }
    }
    fn try_from_optioned(value: VariableOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            expression: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .expression
                    .ok_or(crate::optionable::Error {
                        missing_field: "expression",
                    })?,
            )?,
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: VariableOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.expression {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.expression,
                other_value,
            )?;
        }
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        Ok(())
    }
}
