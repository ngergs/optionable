#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ShareableCapacityStatus reports aggregate amounts for a single shareable capacity key.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ShareableCapacityStatusAc {
    /// Available is Total minus Consumed, never negative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<
        <::k8s_openapi029::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
    /// Consumed is the amount drawn by current allocations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed: Option<
        <::k8s_openapi029::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
    /// Name is the capacity name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// Total is the sum of this capacity across shareable devices in the pool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<
        <::k8s_openapi029::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::resource::v1alpha3::ShareableCapacityStatus {
    type Optioned = ShareableCapacityStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ShareableCapacityStatusAc {
    type Optioned = ShareableCapacityStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::resource::v1alpha3::ShareableCapacityStatus {
    fn into_optioned(self) -> ShareableCapacityStatusAc {
        ShareableCapacityStatusAc {
            available: Some(crate::OptionableConvert::into_optioned(self.available)),
            consumed: Some(crate::OptionableConvert::into_optioned(self.consumed)),
            name: Some(self.name),
            total: Some(crate::OptionableConvert::into_optioned(self.total)),
        }
    }
    fn try_from_optioned(
        value: ShareableCapacityStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            available: crate::OptionableConvert::try_from_optioned(
                value
                    .available
                    .ok_or(crate::Error {
                        missing_field: "available",
                    })?,
            )?,
            consumed: crate::OptionableConvert::try_from_optioned(
                value
                    .consumed
                    .ok_or(crate::Error {
                        missing_field: "consumed",
                    })?,
            )?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            total: crate::OptionableConvert::try_from_optioned(
                value
                    .total
                    .ok_or(crate::Error {
                        missing_field: "total",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: ShareableCapacityStatusAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.available {
            crate::OptionableConvert::merge(&mut self.available, other_value)?;
        }
        if let Some(other_value) = other.consumed {
            crate::OptionableConvert::merge(&mut self.consumed, other_value)?;
        }
        if let Some(other_value) = other.name {
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.total {
            crate::OptionableConvert::merge(&mut self.total, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi029::api::resource::v1alpha3::ShareableCapacityStatus,
> for ShareableCapacityStatusAc {
    fn from_optionable(
        value: k8s_openapi029::api::resource::v1alpha3::ShareableCapacityStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::resource::v1alpha3::ShareableCapacityStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::resource::v1alpha3::ShareableCapacityStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for ShareableCapacityStatusAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.available, other.available);
        k8s_openapi029::DeepMerge::merge_from(&mut self.consumed, other.consumed);
        k8s_openapi029::DeepMerge::merge_from(&mut self.name, other.name);
        k8s_openapi029::DeepMerge::merge_from(&mut self.total, other.total);
    }
}
