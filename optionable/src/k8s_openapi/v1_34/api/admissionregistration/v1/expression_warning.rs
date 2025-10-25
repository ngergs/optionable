pub struct ExpressionWarningOpt {
    pub field_ref: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub warning: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1::expression_warning::ExpressionWarning {
    type Optioned = ExpressionWarningOpt;
}
#[automatically_derived]
impl crate::Optionable for ExpressionWarningOpt {
    type Optioned = ExpressionWarningOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::expression_warning::ExpressionWarning {
    fn into_optioned(self) -> ExpressionWarningOpt {
        ExpressionWarningOpt {
            field_ref: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.field_ref,
                ),
            ),
            warning: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.warning,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: ExpressionWarningOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            field_ref: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .field_ref
                    .ok_or(crate::optionable::Error {
                        missing_field: "field_ref",
                    })?,
            )?,
            warning: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .warning
                    .ok_or(crate::optionable::Error {
                        missing_field: "warning",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ExpressionWarningOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.field_ref {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.field_ref,
                other_value,
            )?;
        }
        if let Some(other_value) = other.warning {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.warning,
                other_value,
            )?;
        }
        Ok(())
    }
}
