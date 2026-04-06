#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VariableAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1beta1::Variable {
    type Optioned = VariableAc;
}
#[automatically_derived]
impl crate::Optionable for VariableAc {
    type Optioned = VariableAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1beta1::Variable {
    fn into_optioned(self) -> VariableAc {
        VariableAc {
            expression: Some(crate::OptionableConvert::into_optioned(self.expression)),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(value: VariableAc) -> Result<Self, crate::Error> {
        Ok(Self {
            expression: crate::OptionableConvert::try_from_optioned(
                value
                    .expression
                    .ok_or(crate::Error {
                        missing_field: "expression",
                    })?,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: VariableAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.expression {
            crate::OptionableConvert::merge(&mut self.expression, other_value)?;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1beta1::Variable,
> for VariableAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1beta1::Variable,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1beta1::Variable,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1beta1::Variable,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
