#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ExpressionWarning is a warning information that targets a specific expression.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExpressionWarningAc {
    /// The path to the field that refers the expression. For example, the reference to the expression of the first item of validations is "spec.validations\[0\].expression"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<std::string::String>,
    /// The content of type checking information in a human-readable form. Each line of the warning contains the type that the expression is checked against, followed by the type check error from the compiler.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1::ExpressionWarning {
    type Optioned = ExpressionWarningAc;
}
#[automatically_derived]
impl crate::Optionable for ExpressionWarningAc {
    type Optioned = ExpressionWarningAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1::ExpressionWarning {
    fn into_optioned(self) -> ExpressionWarningAc {
        ExpressionWarningAc {
            field_ref: Some(self.field_ref),
            warning: Some(self.warning),
        }
    }
    fn try_from_optioned(value: ExpressionWarningAc) -> Result<Self, crate::Error> {
        Ok(Self {
            field_ref: value
                .field_ref
                .ok_or(crate::Error {
                    missing_field: "field_ref",
                })?,
            warning: value
                .warning
                .ok_or(crate::Error {
                    missing_field: "warning",
                })?,
        })
    }
    fn merge(&mut self, other: ExpressionWarningAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.field_ref {
            self.field_ref = other_value;
        }
        if let Some(other_value) = other.warning {
            self.warning = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1::ExpressionWarning,
> for ExpressionWarningAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1::ExpressionWarning,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1::ExpressionWarning,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1::ExpressionWarning,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
