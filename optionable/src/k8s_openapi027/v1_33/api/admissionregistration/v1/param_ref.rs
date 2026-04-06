#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ParamRef describes how to locate the params to be used as input to expressions of rules applied by a policy binding.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ParamRefAc {
    /// name is the name of the resource being referenced.
    ///
    /// One of `name` or `selector` must be set, but `name` and `selector` are mutually exclusive properties. If one is set, the other must be unset.
    ///
    /// A single parameter used for all admission requests can be configured by setting the `name` field, leaving `selector` blank, and setting namespace if `paramKind` is namespace-scoped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// namespace is the namespace of the referenced resource. Allows limiting the search for params to a specific namespace. Applies to both `name` and `selector` fields.
    ///
    /// A per-namespace parameter may be used by specifying a namespace-scoped `paramKind` in the policy and leaving this field empty.
    ///
    /// - If `paramKind` is cluster-scoped, this field MUST be unset. Setting this field results in a configuration error.
    ///
    /// - If `paramKind` is namespace-scoped, the namespace of the object being evaluated for admission will be used when this field is left unset. Take care that if this is left empty the binding must not match any cluster-scoped resources, which will result in an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<std::string::String>,
    /// `parameterNotFoundAction` controls the behavior of the binding when the resource exists, and name or selector is valid, but there are no parameters matched by the binding. If the value is set to `Allow`, then no matched parameters will be treated as successful validation by the binding. If set to `Deny`, then no matched parameters will be subject to the `failurePolicy` of the policy.
    ///
    /// Allowed values are `Allow` or `Deny`
    ///
    /// Required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_not_found_action: Option<std::string::String>,
    /// selector can be used to match multiple param objects based on their labels. Supply selector: {} to match all resources of the ParamKind.
    ///
    /// If multiple params are found, they are all evaluated with the policy expressions and the results are ANDed together.
    ///
    /// One of `name` or `selector` must be set, but `name` and `selector` are mutually exclusive properties. If one is set, the other must be unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::admissionregistration::v1::ParamRef {
    type Optioned = ParamRefAc;
}
#[automatically_derived]
impl crate::Optionable for ParamRefAc {
    type Optioned = ParamRefAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1::ParamRef {
    fn into_optioned(self) -> ParamRefAc {
        ParamRefAc {
            name: self.name,
            namespace: self.namespace,
            parameter_not_found_action: self.parameter_not_found_action,
            selector: crate::OptionableConvert::into_optioned(self.selector),
        }
    }
    fn try_from_optioned(value: ParamRefAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value.name,
            namespace: value.namespace,
            parameter_not_found_action: value.parameter_not_found_action,
            selector: crate::OptionableConvert::try_from_optioned(value.selector)?,
        })
    }
    fn merge(&mut self, other: ParamRefAc) -> Result<(), crate::Error> {
        self.name = other.name;
        self.namespace = other.namespace;
        self.parameter_not_found_action = other.parameter_not_found_action;
        crate::OptionableConvert::merge(&mut self.selector, other.selector)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::admissionregistration::v1::ParamRef>
for ParamRefAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1::ParamRef,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::admissionregistration::v1::ParamRef, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1::ParamRef,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
