#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Variable is the definition of a variable that is used for composition.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VariableAc {
    /// Expression is the expression that will be evaluated as the value of the variable. The CEL expression has access to the same identifiers as the CEL expressions in Validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<std::string::String>,
    /// Name is the name of the variable. The name must be a valid CEL identifier and unique among all variables. The variable can be accessed in other expressions through `variables` For example, if name is "foo", the variable will be available as `variables.foo`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1alpha1::Variable {
    type Optioned = VariableAc;
}
#[automatically_derived]
impl crate::Optionable for VariableAc {
    type Optioned = VariableAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1alpha1::Variable {
    fn into_optioned(self) -> VariableAc {
        VariableAc {
            expression: Some(self.expression),
            name: Some(self.name),
        }
    }
    fn try_from_optioned(value: VariableAc) -> Result<Self, crate::Error> {
        Ok(Self {
            expression: value
                .expression
                .ok_or(crate::Error {
                    missing_field: "expression",
                })?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
        })
    }
    fn merge(&mut self, other: VariableAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.expression {
            self.expression = other_value;
        }
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1alpha1::Variable,
> for VariableAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1alpha1::Variable,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1alpha1::Variable,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1alpha1::Variable,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
