#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// LimitRangeItem defines a min/max usage limit for any resource that matches on kind.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LimitRangeItemAc {
    /// Default resource requirement limit value by resource name if resource limit is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
    /// DefaultRequest is the default resource requirement request value by resource name if resource request is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_request: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
    /// Max usage constraints on this kind by resource name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
    /// MaxLimitRequestRatio if specified, the named resource must have a request and limit that are both non-zero where limit divided by request is less than or equal to the enumerated value; this represents the max burst for the named resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_limit_request_ratio: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
    /// Min usage constraints on this kind by resource name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
    /// Type of resource that this limit applies to.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::LimitRangeItem {
    type Optioned = LimitRangeItemAc;
}
#[automatically_derived]
impl crate::Optionable for LimitRangeItemAc {
    type Optioned = LimitRangeItemAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::LimitRangeItem {
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
            type_: Some(self.type_),
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
            type_: value
                .type_
                .ok_or(crate::Error {
                    missing_field: "type_",
                })?,
        })
    }
    fn merge(&mut self, other: LimitRangeItemAc) -> Result<(), crate::Error> {
        if self.default.is_none() {
            self.default = other.default;
        }
        if let Some(other_value) = other.default {
            crate::OptionableConvert::merge(&mut self.default, other_value)?;
        }
        if self.default_request.is_none() {
            self.default_request = other.default_request;
        }
        if let Some(other_value) = other.default_request {
            crate::OptionableConvert::merge(&mut self.default_request, other_value)?;
        }
        if self.max.is_none() {
            self.max = other.max;
        }
        if let Some(other_value) = other.max {
            crate::OptionableConvert::merge(&mut self.max, other_value)?;
        }
        if self.max_limit_request_ratio.is_none() {
            self.max_limit_request_ratio = other.max_limit_request_ratio;
        }
        if let Some(other_value) = other.max_limit_request_ratio {
            crate::OptionableConvert::merge(
                &mut self.max_limit_request_ratio,
                other_value,
            )?;
        }
        if self.min.is_none() {
            self.min = other.min;
        }
        if let Some(other_value) = other.min {
            crate::OptionableConvert::merge(&mut self.min, other_value)?;
        }
        if let Some(other_value) = other.type_ {
            self.type_ = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::LimitRangeItem>
for LimitRangeItemAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::LimitRangeItem) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::LimitRangeItem, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::LimitRangeItem,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
