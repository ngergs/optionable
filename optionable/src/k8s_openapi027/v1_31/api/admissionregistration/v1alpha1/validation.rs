#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Validation specifies the CEL expression which is used to apply the validation.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ValidationAc {
    /// Expression represents the expression which will be evaluated by CEL. ref: https://github.com/google/cel-spec CEL expressions have access to the contents of the API request/response, organized into CEL variables as well as some other useful variables:
    ///
    /// - 'object' - The object from the incoming request. The value is null for DELETE requests. - 'oldObject' - The existing object. The value is null for CREATE requests. - 'request' - Attributes of the API request(\[ref\](/pkg/apis/admission/types.go#AdmissionRequest)). - 'params' - Parameter resource referred to by the policy binding being evaluated. Only populated if the policy has a ParamKind. - 'namespaceObject' - The namespace object that the incoming object belongs to. The value is null for cluster-scoped resources. - 'variables' - Map of composited variables, from its name to its lazily evaluated value.
    ///   For example, a variable named 'foo' can be accessed as 'variables.foo'.
    /// - 'authorizer' - A CEL Authorizer. May be used to perform authorization checks for the principal (user or service account) of the request.
    ///   See https://pkg.go.dev/k8s.io/apiserver/pkg/cel/library#Authz
    /// - 'authorizer.requestResource' - A CEL ResourceCheck constructed from the 'authorizer' and configured with the
    ///   request resource.
    ///
    /// The `apiVersion`, `kind`, `metadata.name` and `metadata.generateName` are always accessible from the root of the object. No other metadata properties are accessible.
    ///
    /// Only property names of the form `\[a-zA-Z_.-/\]\[a-zA-Z0-9_.-/\]*` are accessible. Accessible property names are escaped according to the following rules when accessed in the expression: - '__' escapes to '__underscores__' - '.' escapes to '__dot__' - '-' escapes to '__dash__' - '/' escapes to '__slash__' - Property names that exactly match a CEL RESERVED keyword escape to '__{keyword}__'. The keywords are:
    ///       "true", "false", "null", "in", "as", "break", "const", "continue", "else", "for", "function", "if",
    ///       "import", "let", "loop", "package", "namespace", "return".
    /// Examples:
    ///   - Expression accessing a property named "namespace": {"Expression": "object.__namespace__ \> 0"}
    ///   - Expression accessing a property named "x-prop": {"Expression": "object.x__dash__prop \> 0"}
    ///   - Expression accessing a property named "redact__d": {"Expression": "object.redact__underscores__d \> 0"}
    ///
    /// Equality on arrays with list type of 'set' or 'map' ignores element order, i.e. \[1, 2\] == \[2, 1\]. Concatenation on arrays with x-kubernetes-list-type use the semantics of the list type:
    ///   - 'set': `X + Y` performs a union where the array positions of all elements in `X` are preserved and
    ///     non-intersecting elements in `Y` are appended, retaining their partial order.
    ///   - 'map': `X + Y` performs a merge where the array positions of all keys in `X` are preserved but the values
    ///     are overwritten by values in `Y` when the key sets of `X` and `Y` intersect. Elements in `Y` with
    ///     non-intersecting keys are appended, retaining their partial order.
    /// Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<std::string::String>,
    /// Message represents the message displayed when validation fails. The message is required if the Expression contains line breaks. The message must not contain line breaks. If unset, the message is "failed rule: {Rule}". e.g. "must be a URL with the host matching spec.host" If the Expression contains line breaks. Message is required. The message must not contain line breaks. If unset, the message is "failed Expression: {Expression}".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// messageExpression declares a CEL expression that evaluates to the validation failure message that is returned when this rule fails. Since messageExpression is used as a failure message, it must evaluate to a string. If both message and messageExpression are present on a validation, then messageExpression will be used if validation fails. If messageExpression results in a runtime error, the runtime error is logged, and the validation failure message is produced as if the messageExpression field were unset. If messageExpression evaluates to an empty string, a string with only spaces, or a string that contains line breaks, then the validation failure message will also be produced as if the messageExpression field were unset, and the fact that messageExpression produced an empty string/string with only spaces/string with line breaks will be logged. messageExpression has access to all the same variables as the `expression` except for 'authorizer' and 'authorizer.requestResource'. Example: "object.x must be less than max ("+string(params.max)+")"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_expression: Option<std::string::String>,
    /// Reason represents a machine-readable description of why this validation failed. If this is the first validation in the list to fail, this reason, as well as the corresponding HTTP response code, are used in the HTTP response to the client. The currently supported reasons are: "Unauthorized", "Forbidden", "Invalid", "RequestEntityTooLarge". If not set, StatusReasonInvalid is used in the response to the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
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
