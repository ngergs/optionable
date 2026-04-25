#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ApplyConfiguration defines the desired configuration values of an object.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ApplyConfigurationAc {
    /// expression will be evaluated by CEL to create an apply configuration. ref: https://github.com/google/cel-spec
    ///
    /// Apply configurations are declared in CEL using object initialization. For example, this CEL expression returns an apply configuration to set a single field:
    ///
    ///   Object{
    ///       spec: Object.spec{
    ///         serviceAccountName: "example"
    ///       }
    ///     }
    ///
    /// Apply configurations may not modify atomic structs, maps or arrays due to the risk of accidental deletion of values not included in the apply configuration.
    ///
    /// CEL expressions have access to the object types needed to create apply configurations:
    ///
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
    /// The `apiVersion`, `kind`, `metadata.name` and `metadata.generateName` are always accessible from the root of the object. No other metadata properties are accessible.
    ///
    /// Only property names of the form `\[a-zA-Z_.-/\]\[a-zA-Z0-9_.-/\]*` are accessible. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1alpha1::ApplyConfiguration {
    type Optioned = ApplyConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for ApplyConfigurationAc {
    type Optioned = ApplyConfigurationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1alpha1::ApplyConfiguration {
    fn into_optioned(self) -> ApplyConfigurationAc {
        ApplyConfigurationAc {
            expression: self.expression,
        }
    }
    fn try_from_optioned(value: ApplyConfigurationAc) -> Result<Self, crate::Error> {
        Ok(Self {
            expression: value.expression,
        })
    }
    fn merge(&mut self, other: ApplyConfigurationAc) -> Result<(), crate::Error> {
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
    k8s_openapi027::api::admissionregistration::v1alpha1::ApplyConfiguration,
> for ApplyConfigurationAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1alpha1::ApplyConfiguration,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1alpha1::ApplyConfiguration,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1alpha1::ApplyConfiguration,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for ApplyConfigurationAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.expression, other.expression);
    }
}
