#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// JSONPatch defines a JSON Patch.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct JSONPatchAc {
    /// expression will be evaluated by CEL to create a \[JSON patch\](https://jsonpatch.com/). ref: https://github.com/google/cel-spec
    ///
    /// expression must return an array of JSONPatch values.
    ///
    /// For example, this CEL expression returns a JSON patch to conditionally modify a value:
    ///
    ///   \[
    ///         JSONPatch{op: "test", path: "/spec/example", value: "Red"},
    ///         JSONPatch{op: "replace", path: "/spec/example", value: "Green"}
    ///       \]
    ///
    /// To define an object for the patch value, use Object types. For example:
    ///
    ///   \[
    ///         JSONPatch{
    ///           op: "add",
    ///           path: "/spec/selector",
    ///           value: Object.spec.selector{matchLabels: {"environment": "test"}}
    ///         }
    ///       \]
    ///
    /// To use strings containing '/' and '~' as JSONPatch path keys, use "jsonpatch.escapeKey". For example:
    ///
    ///   \[
    ///         JSONPatch{
    ///           op: "add",
    ///           path: "/metadata/labels/" + jsonpatch.escapeKey("example.com/environment"),
    ///           value: "test"
    ///         },
    ///       \]
    ///
    /// CEL expressions have access to the types needed to create JSON patches and objects:
    ///
    /// - 'JSONPatch' - CEL type of JSON Patch operations. JSONPatch has the fields 'op', 'from', 'path' and 'value'.
    ///   See \[JSON patch\](https://jsonpatch.com/) for more details. The 'value' field may be set to any of: string,
    ///   integer, array, map or object.  If set, the 'path' and 'from' fields must be set to a
    ///   \[JSON pointer\](https://datatracker.ietf.org/doc/html/rfc6901/) string, where the 'jsonpatch.escapeKey()' CEL
    ///   function may be used to escape path keys containing '/' and '~'.
    /// - 'Object' - CEL type of the resource object. - 'Object.\<fieldName\>' - CEL type of object field (such as 'Object.spec') - 'Object.\<fieldName1\>.\<fieldName2\>...\<fieldNameN\>` - CEL type of nested field (such as 'Object.spec.containers')
    ///
    /// CEL expressions have access to the contents of the API request, organized into CEL variables as well as some other useful variables:
    ///
    /// - 'object' - The object from the incoming request. The value is null for DELETE requests. - 'oldObject' - The existing object. The value is null for CREATE requests. - 'request' - Attributes of the API request(\[ref\](/pkg/apis/admission/types.go#AdmissionRequest)). - 'params' - Parameter resource referred to by the policy binding being evaluated. Only populated if the policy has a ParamKind. - 'namespaceObject' - The namespace object that the incoming object belongs to. The value is null for cluster-scoped resources. - 'variables' - Map of composited variables, from its name to its lazily evaluated value.
    ///   For example, a variable named 'foo' can be accessed as 'variables.foo'.
    /// - 'authorizer' - A CEL Authorizer. May be used to perform authorization checks for the principal (user or service account) of the request.
    ///   See https://pkg.go.dev/k8s.io/apiserver/pkg/cel/library#Authz
    /// - 'authorizer.requestResource' - A CEL ResourceCheck constructed from the 'authorizer' and configured with the
    ///   request resource.
    ///
    /// CEL expressions have access to \[Kubernetes CEL function libraries\](https://kubernetes.io/docs/reference/using-api/cel/#cel-options-language-features-and-libraries) as well as:
    ///
    /// - 'jsonpatch.escapeKey' - Performs JSONPatch key escaping. '~' and  '/' are escaped as '~0' and `~1' respectively).
    ///
    /// Only property names of the form `\[a-zA-Z_.-/\]\[a-zA-Z0-9_.-/\]*` are accessible. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1alpha1::JSONPatch {
    type Optioned = JSONPatchAc;
}
#[automatically_derived]
impl crate::Optionable for JSONPatchAc {
    type Optioned = JSONPatchAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1alpha1::JSONPatch {
    fn into_optioned(self) -> JSONPatchAc {
        JSONPatchAc {
            expression: self.expression,
        }
    }
    fn try_from_optioned(value: JSONPatchAc) -> Result<Self, crate::Error> {
        Ok(Self {
            expression: value.expression,
        })
    }
    fn merge(&mut self, other: JSONPatchAc) -> Result<(), crate::Error> {
        if self.expression.is_none() {
            self.expression = crate::OptionableConvert::try_from_optioned(
                other.expression,
            )?;
        } else if let Some(self_value) = self.expression.as_mut()
            && let Some(other_value) = other.expression
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1alpha1::JSONPatch,
> for JSONPatchAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1alpha1::JSONPatch,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1alpha1::JSONPatch,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1alpha1::JSONPatch,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
