#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExpressionWarningAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1alpha1::ExpressionWarning {
    type Optioned = ExpressionWarningAc;
}
#[automatically_derived]
impl crate::Optionable for ExpressionWarningAc {
    type Optioned = ExpressionWarningAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1alpha1::ExpressionWarning {
    fn into_optioned(self) -> ExpressionWarningAc {
        ExpressionWarningAc {
            field_ref: Some(crate::OptionableConvert::into_optioned(self.field_ref)),
            warning: Some(crate::OptionableConvert::into_optioned(self.warning)),
        }
    }
    fn try_from_optioned(value: ExpressionWarningAc) -> Result<Self, crate::Error> {
        Ok(Self {
            field_ref: crate::OptionableConvert::try_from_optioned(
                value
                    .field_ref
                    .ok_or(crate::Error {
                        missing_field: "field_ref",
                    })?,
            )?,
            warning: crate::OptionableConvert::try_from_optioned(
                value
                    .warning
                    .ok_or(crate::Error {
                        missing_field: "warning",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: ExpressionWarningAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.field_ref {
            crate::OptionableConvert::merge(&mut self.field_ref, other_value)?;
        }
        if let Some(other_value) = other.warning {
            crate::OptionableConvert::merge(&mut self.warning, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1alpha1::ExpressionWarning,
> for ExpressionWarningAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1alpha1::ExpressionWarning,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1alpha1::ExpressionWarning,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1alpha1::ExpressionWarning,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
