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
    pub expression: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_expression: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1alpha1::Validation {
    type Optioned = ValidationAc;
}
#[automatically_derived]
impl crate::Optionable for ValidationAc {
    type Optioned = ValidationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1alpha1::Validation {
    fn into_optioned(self) -> ValidationAc {
        ValidationAc {
            expression: Some(crate::OptionableConvert::into_optioned(self.expression)),
            message: crate::OptionableConvert::into_optioned(self.message),
            message_expression: crate::OptionableConvert::into_optioned(
                self.message_expression,
            ),
            reason: crate::OptionableConvert::into_optioned(self.reason),
        }
    }
    fn try_from_optioned(value: ValidationAc) -> Result<Self, crate::Error> {
        Ok(Self {
            expression: crate::OptionableConvert::try_from_optioned(
                value
                    .expression
                    .ok_or(crate::Error {
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
    fn merge(&mut self, other: ValidationAc) -> Result<(), crate::Error> {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1alpha1::Validation,
> for ValidationAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1alpha1::Validation,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1alpha1::Validation,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1alpha1::Validation,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
