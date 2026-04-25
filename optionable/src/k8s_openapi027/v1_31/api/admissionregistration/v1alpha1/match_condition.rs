#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MatchConditionAc {
    /// Expression represents the expression which will be evaluated by CEL. Must evaluate to bool. CEL expressions have access to the contents of the AdmissionRequest and Authorizer, organized into CEL variables:
    ///
    /// 'object' - The object from the incoming request. The value is null for DELETE requests. 'oldObject' - The existing object. The value is null for CREATE requests. 'request' - Attributes of the admission request(/pkg/apis/admission/types.go#AdmissionRequest). 'authorizer' - A CEL Authorizer. May be used to perform authorization checks for the principal (user or service account) of the request.
    ///   See https://pkg.go.dev/k8s.io/apiserver/pkg/cel/library#Authz
    /// 'authorizer.requestResource' - A CEL ResourceCheck constructed from the 'authorizer' and configured with the
    ///   request resource.
    /// Documentation on CEL: https://kubernetes.io/docs/reference/using-api/cel/
    ///
    /// Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<std::string::String>,
    /// Name is an identifier for this match condition, used for strategic merging of MatchConditions, as well as providing an identifier for logging purposes. A good name should be descriptive of the associated expression. Name must be a qualified name consisting of alphanumeric characters, '-', '_' or '.', and must start and end with an alphanumeric character (e.g. 'MyName',  or 'my.name',  or '123-abc', regex used for validation is '(\[A-Za-z0-9\]\[-A-Za-z0-9_.\]*)?\[A-Za-z0-9\]') with an optional DNS subdomain prefix and '/' (e.g. 'example.com/MyName')
    ///
    /// Required.
    pub name: std::string::String,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1alpha1::MatchCondition {
    type Optioned = MatchConditionAc;
}
#[automatically_derived]
impl crate::Optionable for MatchConditionAc {
    type Optioned = MatchConditionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1alpha1::MatchCondition {
    fn into_optioned(self) -> MatchConditionAc {
        MatchConditionAc {
            expression: Some(self.expression),
            name: self.name,
        }
    }
    fn try_from_optioned(value: MatchConditionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            expression: value
                .expression
                .ok_or(crate::Error {
                    missing_field: "expression",
                })?,
            name: value.name,
        })
    }
    fn merge(&mut self, other: MatchConditionAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.expression {
            self.expression = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        self.name = other.name;
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi027::api::admissionregistration::v1alpha1::MatchCondition {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.name == other.name
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1alpha1::MatchCondition,
> for MatchConditionAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1alpha1::MatchCondition,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1alpha1::MatchCondition,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1alpha1::MatchCondition,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for MatchConditionAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.expression, other.expression);
        k8s_openapi027::DeepMerge::merge_from(&mut self.name, other.name);
    }
}
impl crate::merge::MapKeysEq for MatchConditionAc {
    fn keys_eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
