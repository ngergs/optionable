#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// AuditAnnotation describes how to produce an audit annotation for an API request.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AuditAnnotationAc {
    /// key specifies the audit annotation key. The audit annotation keys of a ValidatingAdmissionPolicy must be unique. The key must be a qualified name (\[A-Za-z0-9\]\[-A-Za-z0-9_.\]*) no more than 63 bytes in length.
    ///
    /// The key is combined with the resource name of the ValidatingAdmissionPolicy to construct an audit annotation key: "{ValidatingAdmissionPolicy name}/{key}".
    ///
    /// If an admission webhook uses the same resource name as this ValidatingAdmissionPolicy and the same audit annotation key, the annotation key will be identical. In this case, the first annotation written with the key will be included in the audit event and all subsequent annotations with the same key will be discarded.
    ///
    /// Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<std::string::String>,
    /// valueExpression represents the expression which is evaluated by CEL to produce an audit annotation value. The expression must evaluate to either a string or null value. If the expression evaluates to a string, the audit annotation is included with the string value. If the expression evaluates to null or empty string the audit annotation will be omitted. The valueExpression may be no longer than 5kb in length. If the result of the valueExpression is more than 10kb in length, it will be truncated to 10kb.
    ///
    /// If multiple ValidatingAdmissionPolicyBinding resources match an API request, then the valueExpression will be evaluated for each binding. All unique values produced by the valueExpressions will be joined together in a comma-separated list.
    ///
    /// Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_expression: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1::AuditAnnotation {
    type Optioned = AuditAnnotationAc;
}
#[automatically_derived]
impl crate::Optionable for AuditAnnotationAc {
    type Optioned = AuditAnnotationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1::AuditAnnotation {
    fn into_optioned(self) -> AuditAnnotationAc {
        AuditAnnotationAc {
            key: Some(self.key),
            value_expression: Some(self.value_expression),
        }
    }
    fn try_from_optioned(value: AuditAnnotationAc) -> Result<Self, crate::Error> {
        Ok(Self {
            key: value
                .key
                .ok_or(crate::Error {
                    missing_field: "key",
                })?,
            value_expression: value
                .value_expression
                .ok_or(crate::Error {
                    missing_field: "value_expression",
                })?,
        })
    }
    fn merge(&mut self, other: AuditAnnotationAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.key {
            self.key = other_value;
        }
        if let Some(other_value) = other.value_expression {
            self.value_expression = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1::AuditAnnotation,
> for AuditAnnotationAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1::AuditAnnotation,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1::AuditAnnotation,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1::AuditAnnotation,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
