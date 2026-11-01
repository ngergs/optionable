#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PartitionTypeStatus reports allocatability for a single partition type, identified by the value of a grouping attribute.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PartitionTypeStatusAc {
    /// Allocatable is the number of additional devices of this partition type that could still be allocated given current shared-counter consumption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocatable: Option<i32>,
    /// Attribute is the fully qualified name of the device attribute whose value groups this entry. It is the PartitionTypeAttribute declared by the devices' own slice, or the default named in the request when their slice declares none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<std::string::String>,
    /// Total is the number of devices of this partition type in the pool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// Type is the partition type value (e.g. "Full" or "Half").
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::resource::v1alpha3::PartitionTypeStatus {
    type Optioned = PartitionTypeStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PartitionTypeStatusAc {
    type Optioned = PartitionTypeStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::resource::v1alpha3::PartitionTypeStatus {
    fn into_optioned(self) -> PartitionTypeStatusAc {
        PartitionTypeStatusAc {
            allocatable: Some(self.allocatable),
            attribute: Some(self.attribute),
            total: Some(self.total),
            type_: Some(self.type_),
        }
    }
    fn try_from_optioned(value: PartitionTypeStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            allocatable: value
                .allocatable
                .ok_or(crate::Error {
                    missing_field: "allocatable",
                })?,
            attribute: value
                .attribute
                .ok_or(crate::Error {
                    missing_field: "attribute",
                })?,
            total: value
                .total
                .ok_or(crate::Error {
                    missing_field: "total",
                })?,
            type_: value
                .type_
                .ok_or(crate::Error {
                    missing_field: "type_",
                })?,
        })
    }
    fn merge(&mut self, other: PartitionTypeStatusAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.allocatable {
            self.allocatable = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.attribute {
            self.attribute = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.total {
            self.total = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.type_ {
            self.type_ = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::resource::v1alpha3::PartitionTypeStatus>
for PartitionTypeStatusAc {
    fn from_optionable(
        value: k8s_openapi029::api::resource::v1alpha3::PartitionTypeStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::resource::v1alpha3::PartitionTypeStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::resource::v1alpha3::PartitionTypeStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for PartitionTypeStatusAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.allocatable, other.allocatable);
        k8s_openapi029::DeepMerge::merge_from(&mut self.attribute, other.attribute);
        k8s_openapi029::DeepMerge::merge_from(&mut self.total, other.total);
        k8s_openapi029::DeepMerge::merge_from(&mut self.type_, other.type_);
    }
}
