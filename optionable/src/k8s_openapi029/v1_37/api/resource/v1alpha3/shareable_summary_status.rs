#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ShareableSummaryStatus reports aggregate capacity for a pool that contains devices with AllowMultipleAllocations.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ShareableSummaryStatusAc {
    /// Capacity reports aggregate total, consumed, and available amounts per shareable capacity key across the pool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<
        std::vec::Vec<
            <::k8s_openapi029::api::resource::v1alpha3::ShareableCapacityStatus as crate::Optionable>::Optioned,
        >,
    >,
    /// FullyAvailableDevices is the number of shareable devices with no capacity consumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fully_available_devices: Option<i32>,
    /// PartiallyAvailableDevices is the number of shareable devices with some but not all capacity consumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partially_available_devices: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::resource::v1alpha3::ShareableSummaryStatus {
    type Optioned = ShareableSummaryStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ShareableSummaryStatusAc {
    type Optioned = ShareableSummaryStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::resource::v1alpha3::ShareableSummaryStatus {
    fn into_optioned(self) -> ShareableSummaryStatusAc {
        ShareableSummaryStatusAc {
            capacity: crate::OptionableConvert::into_optioned(self.capacity),
            fully_available_devices: Some(self.fully_available_devices),
            partially_available_devices: Some(self.partially_available_devices),
        }
    }
    fn try_from_optioned(value: ShareableSummaryStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            capacity: crate::OptionableConvert::try_from_optioned(value.capacity)?,
            fully_available_devices: value
                .fully_available_devices
                .ok_or(crate::Error {
                    missing_field: "fully_available_devices",
                })?,
            partially_available_devices: value
                .partially_available_devices
                .ok_or(crate::Error {
                    missing_field: "partially_available_devices",
                })?,
        })
    }
    fn merge(&mut self, other: ShareableSummaryStatusAc) -> Result<(), crate::Error> {
        if self.capacity.is_none() {
            self.capacity = crate::OptionableConvert::try_from_optioned(other.capacity)?;
        } else if let Some(self_value) = self.capacity.as_mut()
            && let Some(other_value) = other.capacity
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.fully_available_devices {
            self.fully_available_devices = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if let Some(other_value) = other.partially_available_devices {
            self.partially_available_devices = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi029::api::resource::v1alpha3::ShareableSummaryStatus,
> for ShareableSummaryStatusAc {
    fn from_optionable(
        value: k8s_openapi029::api::resource::v1alpha3::ShareableSummaryStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::resource::v1alpha3::ShareableSummaryStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::resource::v1alpha3::ShareableSummaryStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for ShareableSummaryStatusAc {
    fn merge_from(&mut self, other: Self) {
        self.capacity = other.capacity;
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.fully_available_devices,
            other.fully_available_devices,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.partially_available_devices,
            other.partially_available_devices,
        );
    }
}
