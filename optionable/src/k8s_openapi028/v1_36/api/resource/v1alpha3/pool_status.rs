#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PoolStatus contains status information for a single resource pool.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PoolStatusAc {
    /// AllocatedDevices is the number of devices currently allocated to claims. A value of 0 means no devices are allocated. May be unset when validationError is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_devices: Option<i32>,
    /// AvailableDevices is the number of devices available for allocation. This equals TotalDevices - AllocatedDevices - UnavailableDevices. A value of 0 means no devices are currently available. May be unset when validationError is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_devices: Option<i32>,
    /// Driver is the DRA driver name for this pool. Must be a DNS subdomain (e.g., "gpu.example.com").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<std::string::String>,
    /// Generation is the pool generation observed across all ResourceSlices in this pool. Only the latest generation is reported. During a generation rollout, if not all slices at the latest generation have been published, the pool is included with a validationError and device counts unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation: Option<i64>,
    /// NodeName is the node this pool is associated with. When omitted, the pool is not associated with a specific node. Must be a valid DNS subdomain name (RFC1123).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<std::string::String>,
    /// PoolName is the name of the pool. Must be a valid resource pool name (DNS subdomains separated by "/").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_name: Option<std::string::String>,
    /// ResourceSliceCount is the number of ResourceSlices that make up this pool. May be unset when validationError is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_slice_count: Option<i32>,
    /// TotalDevices is the total number of devices in the pool across all slices. A value of 0 means the pool has no devices. May be unset when validationError is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_devices: Option<i32>,
    /// UnavailableDevices is the number of devices that are not available due to taints or other conditions, but are not allocated. A value of 0 means all unallocated devices are available. May be unset when validationError is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable_devices: Option<i32>,
    /// ValidationError is set when the pool's data could not be fully validated (e.g., incomplete slice publication). When set, device count fields and ResourceSliceCount may be unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_error: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi028::api::resource::v1alpha3::PoolStatus {
    type Optioned = PoolStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PoolStatusAc {
    type Optioned = PoolStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi028::api::resource::v1alpha3::PoolStatus {
    fn into_optioned(self) -> PoolStatusAc {
        PoolStatusAc {
            allocated_devices: self.allocated_devices,
            available_devices: self.available_devices,
            driver: Some(self.driver),
            generation: Some(self.generation),
            node_name: self.node_name,
            pool_name: Some(self.pool_name),
            resource_slice_count: self.resource_slice_count,
            total_devices: self.total_devices,
            unavailable_devices: self.unavailable_devices,
            validation_error: self.validation_error,
        }
    }
    fn try_from_optioned(value: PoolStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            allocated_devices: value.allocated_devices,
            available_devices: value.available_devices,
            driver: value
                .driver
                .ok_or(crate::Error {
                    missing_field: "driver",
                })?,
            generation: value
                .generation
                .ok_or(crate::Error {
                    missing_field: "generation",
                })?,
            node_name: value.node_name,
            pool_name: value
                .pool_name
                .ok_or(crate::Error {
                    missing_field: "pool_name",
                })?,
            resource_slice_count: value.resource_slice_count,
            total_devices: value.total_devices,
            unavailable_devices: value.unavailable_devices,
            validation_error: value.validation_error,
        })
    }
    fn merge(&mut self, other: PoolStatusAc) -> Result<(), crate::Error> {
        if self.allocated_devices.is_none() {
            self.allocated_devices = crate::OptionableConvert::try_from_optioned(
                other.allocated_devices,
            )?;
        } else if let Some(self_value) = self.allocated_devices.as_mut()
            && let Some(other_value) = other.allocated_devices
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.available_devices.is_none() {
            self.available_devices = crate::OptionableConvert::try_from_optioned(
                other.available_devices,
            )?;
        } else if let Some(self_value) = self.available_devices.as_mut()
            && let Some(other_value) = other.available_devices
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.driver {
            self.driver = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.generation {
            self.generation = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.node_name.is_none() {
            self.node_name = crate::OptionableConvert::try_from_optioned(
                other.node_name,
            )?;
        } else if let Some(self_value) = self.node_name.as_mut()
            && let Some(other_value) = other.node_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.pool_name {
            self.pool_name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.resource_slice_count.is_none() {
            self.resource_slice_count = crate::OptionableConvert::try_from_optioned(
                other.resource_slice_count,
            )?;
        } else if let Some(self_value) = self.resource_slice_count.as_mut()
            && let Some(other_value) = other.resource_slice_count
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.total_devices.is_none() {
            self.total_devices = crate::OptionableConvert::try_from_optioned(
                other.total_devices,
            )?;
        } else if let Some(self_value) = self.total_devices.as_mut()
            && let Some(other_value) = other.total_devices
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.unavailable_devices.is_none() {
            self.unavailable_devices = crate::OptionableConvert::try_from_optioned(
                other.unavailable_devices,
            )?;
        } else if let Some(self_value) = self.unavailable_devices.as_mut()
            && let Some(other_value) = other.unavailable_devices
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.validation_error.is_none() {
            self.validation_error = crate::OptionableConvert::try_from_optioned(
                other.validation_error,
            )?;
        } else if let Some(self_value) = self.validation_error.as_mut()
            && let Some(other_value) = other.validation_error
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi028::api::resource::v1alpha3::PoolStatus>
for PoolStatusAc {
    fn from_optionable(
        value: k8s_openapi028::api::resource::v1alpha3::PoolStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi028::api::resource::v1alpha3::PoolStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::api::resource::v1alpha3::PoolStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::DeepMerge for PoolStatusAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.allocated_devices,
            other.allocated_devices,
        );
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.available_devices,
            other.available_devices,
        );
        k8s_openapi028::DeepMerge::merge_from(&mut self.driver, other.driver);
        k8s_openapi028::DeepMerge::merge_from(&mut self.generation, other.generation);
        k8s_openapi028::DeepMerge::merge_from(&mut self.node_name, other.node_name);
        k8s_openapi028::DeepMerge::merge_from(&mut self.pool_name, other.pool_name);
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.resource_slice_count,
            other.resource_slice_count,
        );
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.total_devices,
            other.total_devices,
        );
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.unavailable_devices,
            other.unavailable_devices,
        );
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.validation_error,
            other.validation_error,
        );
    }
}
