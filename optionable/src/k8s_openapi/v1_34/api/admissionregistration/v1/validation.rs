pub struct ValidationOpt {
    pub expression: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub message_expression: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::admissionregistration::v1::Validation {
    type Optioned = ValidationOpt;
}
#[automatically_derived]
impl crate::Optionable for ValidationOpt {
    type Optioned = ValidationOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::Validation {
    fn into_optioned(self) -> ValidationOpt {
        ValidationOpt {
            expression: Some(crate::OptionableConvert::into_optioned(self.expression)),
            message: crate::OptionableConvert::into_optioned(self.message),
            message_expression: crate::OptionableConvert::into_optioned(
                self.message_expression,
            ),
            reason: crate::OptionableConvert::into_optioned(self.reason),
        }
    }
    fn try_from_optioned(
        value: ValidationOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            expression: crate::OptionableConvert::try_from_optioned(
                value
                    .expression
                    .ok_or(crate::optionable::Error {
                        missing_field: "expression",
                    })?,
            )?,
            message: crate::OptionableConvert::try_from_optioned(value.message)?,
            message_expression: crate::OptionableConvert::try_from_optioned(
                value.message_expression,
            )?,
            reason: crate::OptionableConvert::try_from_optioned(value.reason)?,
        })
    }
    fn merge(&mut self, other: ValidationOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.expression {
            crate::OptionableConvert::merge(&mut self.expression, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.message, other.message)?;
        crate::OptionableConvert::merge(
            &mut self.message_expression,
            other.message_expression,
        )?;
        crate::OptionableConvert::merge(&mut self.reason, other.reason)?;
        Ok(())
    }
}
