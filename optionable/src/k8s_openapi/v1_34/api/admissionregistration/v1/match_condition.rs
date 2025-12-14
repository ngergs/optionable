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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1::MatchCondition {
    type Optioned = MatchConditionAc;
}
#[automatically_derived]
impl crate::Optionable for MatchConditionAc {
    type Optioned = MatchConditionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::MatchCondition {
    fn into_optioned(self) -> MatchConditionAc {
        MatchConditionAc {
            expression: Some(crate::OptionableConvert::into_optioned(self.expression)),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(value: MatchConditionAc) -> Result<Self, crate::Error> {
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
    fn merge(&mut self, other: MatchConditionAc) -> Result<(), crate::Error> {
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
    ::k8s_openapi::api::admissionregistration::v1::MatchCondition,
> for MatchConditionAc {
    fn from_optionable(
        value: ::k8s_openapi::api::admissionregistration::v1::MatchCondition,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::admissionregistration::v1::MatchCondition,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::admissionregistration::v1::MatchCondition,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
