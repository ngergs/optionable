#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TopologySelectorLabelRequirementAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::api::core::v1::TopologySelectorLabelRequirement {
    type Optioned = TopologySelectorLabelRequirementAc;
}
#[automatically_derived]
impl crate::Optionable for TopologySelectorLabelRequirementAc {
    type Optioned = TopologySelectorLabelRequirementAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::core::v1::TopologySelectorLabelRequirement {
    fn into_optioned(self) -> TopologySelectorLabelRequirementAc {
        TopologySelectorLabelRequirementAc {
            key: Some(crate::OptionableConvert::into_optioned(self.key)),
            values: Some(crate::OptionableConvert::into_optioned(self.values)),
        }
    }
    fn try_from_optioned(
        value: TopologySelectorLabelRequirementAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            key: crate::OptionableConvert::try_from_optioned(
                value
                    .key
                    .ok_or(crate::Error {
                        missing_field: "key",
                    })?,
            )?,
            values: crate::OptionableConvert::try_from_optioned(
                value
                    .values
                    .ok_or(crate::Error {
                        missing_field: "values",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: TopologySelectorLabelRequirementAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.key {
            crate::OptionableConvert::merge(&mut self.key, other_value)?;
        }
        if let Some(other_value) = other.values {
            crate::OptionableConvert::merge(&mut self.values, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::api::core::v1::TopologySelectorLabelRequirement,
> for TopologySelectorLabelRequirementAc {
    fn from_optionable(
        value: k8s_openapi026::api::core::v1::TopologySelectorLabelRequirement,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::core::v1::TopologySelectorLabelRequirement,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::core::v1::TopologySelectorLabelRequirement,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
