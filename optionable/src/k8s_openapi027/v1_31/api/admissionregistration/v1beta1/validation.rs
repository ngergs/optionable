#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ValidationAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_expression: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1beta1::Validation {
    type Optioned = ValidationAc;
}
#[automatically_derived]
impl crate::Optionable for ValidationAc {
    type Optioned = ValidationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1beta1::Validation {
    fn into_optioned(self) -> ValidationAc {
        ValidationAc {
            expression: Some(self.expression),
            message: self.message,
            message_expression: self.message_expression,
            reason: self.reason,
        }
    }
    fn try_from_optioned(value: ValidationAc) -> Result<Self, crate::Error> {
        Ok(Self {
            expression: value
                .expression
                .ok_or(crate::Error {
                    missing_field: "expression",
                })?,
            message: value.message,
            message_expression: value.message_expression,
            reason: value.reason,
        })
    }
    fn merge(&mut self, other: ValidationAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.expression {
            self.expression = other_value;
        }
        self.message = other.message;
        self.message_expression = other.message_expression;
        self.reason = other.reason;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1beta1::Validation,
> for ValidationAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1beta1::Validation,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1beta1::Validation,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1beta1::Validation,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
