#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct ScopedResourceSelectorRequirementAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::ScopedResourceSelectorRequirement {
    type Optioned = ScopedResourceSelectorRequirementAc;
}
#[automatically_derived]
impl crate::Optionable for ScopedResourceSelectorRequirementAc {
    type Optioned = ScopedResourceSelectorRequirementAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::ScopedResourceSelectorRequirement {
    fn into_optioned(self) -> ScopedResourceSelectorRequirementAc {
        ScopedResourceSelectorRequirementAc {
            operator: Some(crate::OptionableConvert::into_optioned(self.operator)),
            scope_name: Some(crate::OptionableConvert::into_optioned(self.scope_name)),
            values: crate::OptionableConvert::into_optioned(self.values),
        }
    }
    fn try_from_optioned(
        value: ScopedResourceSelectorRequirementAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            operator: crate::OptionableConvert::try_from_optioned(
                value
                    .operator
                    .ok_or(crate::Error {
                        missing_field: "operator",
                    })?,
            )?,
            scope_name: crate::OptionableConvert::try_from_optioned(
                value
                    .scope_name
                    .ok_or(crate::Error {
                        missing_field: "scope_name",
                    })?,
            )?,
            values: crate::OptionableConvert::try_from_optioned(value.values)?,
        })
    }
    fn merge(
        &mut self,
        other: ScopedResourceSelectorRequirementAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.operator {
            crate::OptionableConvert::merge(&mut self.operator, other_value)?;
        }
        if let Some(other_value) = other.scope_name {
            crate::OptionableConvert::merge(&mut self.scope_name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.values, other.values)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::api::core::v1::ScopedResourceSelectorRequirement,
> for ScopedResourceSelectorRequirementAc {
    fn from_optionable(
        value: ::k8s_openapi::api::core::v1::ScopedResourceSelectorRequirement,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::core::v1::ScopedResourceSelectorRequirement,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::ScopedResourceSelectorRequirement,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
