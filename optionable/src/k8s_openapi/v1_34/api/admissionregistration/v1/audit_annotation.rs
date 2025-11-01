pub struct AuditAnnotationAc {
    pub key: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub value_expression: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1::AuditAnnotation {
    type Optioned = AuditAnnotationAc;
}
#[automatically_derived]
impl crate::Optionable for AuditAnnotationAc {
    type Optioned = AuditAnnotationAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::AuditAnnotation {
    fn into_optioned(self) -> AuditAnnotationAc {
        AuditAnnotationAc {
            key: Some(crate::OptionableConvert::into_optioned(self.key)),
            value_expression: Some(
                crate::OptionableConvert::into_optioned(self.value_expression),
            ),
        }
    }
    fn try_from_optioned(
        value: AuditAnnotationAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            key: crate::OptionableConvert::try_from_optioned(
                value
                    .key
                    .ok_or(crate::optionable::Error {
                        missing_field: "key",
                    })?,
            )?,
            value_expression: crate::OptionableConvert::try_from_optioned(
                value
                    .value_expression
                    .ok_or(crate::optionable::Error {
                        missing_field: "value_expression",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: AuditAnnotationAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.key {
            crate::OptionableConvert::merge(&mut self.key, other_value)?;
        }
        if let Some(other_value) = other.value_expression {
            crate::OptionableConvert::merge(&mut self.value_expression, other_value)?;
        }
        Ok(())
    }
}
