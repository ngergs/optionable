#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AuditAnnotationAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_expression: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1beta1::AuditAnnotation {
    type Optioned = AuditAnnotationAc;
}
#[automatically_derived]
impl crate::Optionable for AuditAnnotationAc {
    type Optioned = AuditAnnotationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1beta1::AuditAnnotation {
    fn into_optioned(self) -> AuditAnnotationAc {
        AuditAnnotationAc {
            key: Some(crate::OptionableConvert::into_optioned(self.key)),
            value_expression: Some(
                crate::OptionableConvert::into_optioned(self.value_expression),
            ),
        }
    }
    fn try_from_optioned(value: AuditAnnotationAc) -> Result<Self, crate::Error> {
        Ok(Self {
            key: crate::OptionableConvert::try_from_optioned(
                value
                    .key
                    .ok_or(crate::Error {
                        missing_field: "key",
                    })?,
            )?,
            value_expression: crate::OptionableConvert::try_from_optioned(
                value
                    .value_expression
                    .ok_or(crate::Error {
                        missing_field: "value_expression",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: AuditAnnotationAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.key {
            crate::OptionableConvert::merge(&mut self.key, other_value)?;
        }
        if let Some(other_value) = other.value_expression {
            crate::OptionableConvert::merge(&mut self.value_expression, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::api::admissionregistration::v1beta1::AuditAnnotation,
> for AuditAnnotationAc {
    fn from_optionable(
        value: ::k8s_openapi::api::admissionregistration::v1beta1::AuditAnnotation,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::admissionregistration::v1beta1::AuditAnnotation,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::admissionregistration::v1beta1::AuditAnnotation,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
