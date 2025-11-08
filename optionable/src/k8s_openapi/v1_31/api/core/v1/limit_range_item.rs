#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct LimitRangeItemAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_request: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_limit_request_ratio: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::LimitRangeItem {
    type Optioned = LimitRangeItemAc;
}
#[automatically_derived]
impl crate::Optionable for LimitRangeItemAc {
    type Optioned = LimitRangeItemAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::LimitRangeItem {
    fn into_optioned(self) -> LimitRangeItemAc {
        LimitRangeItemAc {
            default: crate::OptionableConvert::into_optioned(self.default),
            default_request: crate::OptionableConvert::into_optioned(
                self.default_request,
            ),
            max: crate::OptionableConvert::into_optioned(self.max),
            max_limit_request_ratio: crate::OptionableConvert::into_optioned(
                self.max_limit_request_ratio,
            ),
            min: crate::OptionableConvert::into_optioned(self.min),
            type_: Some(crate::OptionableConvert::into_optioned(self.type_)),
        }
    }
    fn try_from_optioned(value: LimitRangeItemAc) -> Result<Self, crate::Error> {
        Ok(Self {
            default: crate::OptionableConvert::try_from_optioned(value.default)?,
            default_request: crate::OptionableConvert::try_from_optioned(
                value.default_request,
            )?,
            max: crate::OptionableConvert::try_from_optioned(value.max)?,
            max_limit_request_ratio: crate::OptionableConvert::try_from_optioned(
                value.max_limit_request_ratio,
            )?,
            min: crate::OptionableConvert::try_from_optioned(value.min)?,
            type_: crate::OptionableConvert::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::Error {
                        missing_field: "type_",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: LimitRangeItemAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.default, other.default)?;
        crate::OptionableConvert::merge(
            &mut self.default_request,
            other.default_request,
        )?;
        crate::OptionableConvert::merge(&mut self.max, other.max)?;
        crate::OptionableConvert::merge(
            &mut self.max_limit_request_ratio,
            other.max_limit_request_ratio,
        )?;
        crate::OptionableConvert::merge(&mut self.min, other.min)?;
        if let Some(other_value) = other.type_ {
            crate::OptionableConvert::merge(&mut self.type_, other_value)?;
        }
        Ok(())
    }
}
