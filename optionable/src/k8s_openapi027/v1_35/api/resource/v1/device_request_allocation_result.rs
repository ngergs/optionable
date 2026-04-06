#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DeviceRequestAllocationResult contains the allocation result for one request.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceRequestAllocationResultAc {
    /// AdminAccess indicates that this device was allocated for administrative access. See the corresponding request field for a definition of mode.
    ///
    /// This is an alpha field and requires enabling the DRAAdminAccess feature gate. Admin access is disabled if this field is unset or set to false, otherwise it is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_access: Option<bool>,
    /// BindingConditions contains a copy of the BindingConditions from the corresponding ResourceSlice at the time of allocation.
    ///
    /// This is an alpha field and requires enabling the DRADeviceBindingConditions and DRAResourceClaimDeviceStatus feature gates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_conditions: Option<std::vec::Vec<std::string::String>>,
    /// BindingFailureConditions contains a copy of the BindingFailureConditions from the corresponding ResourceSlice at the time of allocation.
    ///
    /// This is an alpha field and requires enabling the DRADeviceBindingConditions and DRAResourceClaimDeviceStatus feature gates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_failure_conditions: Option<std::vec::Vec<std::string::String>>,
    /// ConsumedCapacity tracks the amount of capacity consumed per device as part of the claim request. The consumed amount may differ from the requested amount: it is rounded up to the nearest valid value based on the device’s requestPolicy if applicable (i.e., may not be less than the requested amount).
    ///
    /// The total consumed capacity for each device must not exceed the DeviceCapacity's Value.
    ///
    /// This field is populated only for devices that allow multiple allocations. All capacity entries are included, even if the consumed amount is zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
    /// Device references one device instance via its name in the driver's resource pool. It must be a DNS label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<std::string::String>,
    /// Driver specifies the name of the DRA driver whose kubelet plugin should be invoked to process the allocation once the claim is needed on a node.
    ///
    /// Must be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver. It should use only lower case characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<std::string::String>,
    /// This name together with the driver name and the device name field identify which device was allocated (`\<driver name\>/\<pool name\>/\<device name\>`).
    ///
    /// Must not be longer than 253 characters and may contain one or more DNS sub-domains separated by slashes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<std::string::String>,
    /// Request is the name of the request in the claim which caused this device to be allocated. If it references a subrequest in the firstAvailable list on a DeviceRequest, this field must include both the name of the main request and the subrequest using the format \<main request\>/\<subrequest\>.
    ///
    /// Multiple devices may have been allocated per request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<std::string::String>,
    /// ShareID uniquely identifies an individual allocation share of the device, used when the device supports multiple simultaneous allocations. It serves as an additional map key to differentiate concurrent shares of the same device.
    #[serde(rename = "shareID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_id: Option<std::string::String>,
    /// A copy of all tolerations specified in the request at the time when the device got allocated.
    ///
    /// The maximum number of tolerations is 16.
    ///
    /// This is an alpha field and requires enabling the DRADeviceTaints feature gate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1::DeviceToleration as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::resource::v1::DeviceRequestAllocationResult {
    type Optioned = DeviceRequestAllocationResultAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceRequestAllocationResultAc {
    type Optioned = DeviceRequestAllocationResultAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1::DeviceRequestAllocationResult {
    fn into_optioned(self) -> DeviceRequestAllocationResultAc {
        DeviceRequestAllocationResultAc {
            admin_access: self.admin_access,
            binding_conditions: self.binding_conditions,
            binding_failure_conditions: self.binding_failure_conditions,
            consumed_capacity: crate::OptionableConvert::into_optioned(
                self.consumed_capacity,
            ),
            device: Some(self.device),
            driver: Some(self.driver),
            pool: Some(self.pool),
            request: Some(self.request),
            share_id: self.share_id,
            tolerations: crate::OptionableConvert::into_optioned(self.tolerations),
        }
    }
    fn try_from_optioned(
        value: DeviceRequestAllocationResultAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            admin_access: value.admin_access,
            binding_conditions: value.binding_conditions,
            binding_failure_conditions: value.binding_failure_conditions,
            consumed_capacity: crate::OptionableConvert::try_from_optioned(
                value.consumed_capacity,
            )?,
            device: value
                .device
                .ok_or(crate::Error {
                    missing_field: "device",
                })?,
            driver: value
                .driver
                .ok_or(crate::Error {
                    missing_field: "driver",
                })?,
            pool: value
                .pool
                .ok_or(crate::Error {
                    missing_field: "pool",
                })?,
            request: value
                .request
                .ok_or(crate::Error {
                    missing_field: "request",
                })?,
            share_id: value.share_id,
            tolerations: crate::OptionableConvert::try_from_optioned(value.tolerations)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceRequestAllocationResultAc,
    ) -> Result<(), crate::Error> {
        self.admin_access = other.admin_access;
        self.binding_conditions = other.binding_conditions;
        self.binding_failure_conditions = other.binding_failure_conditions;
        crate::OptionableConvert::merge(
            &mut self.consumed_capacity,
            other.consumed_capacity,
        )?;
        if let Some(other_value) = other.device {
            self.device = other_value;
        }
        if let Some(other_value) = other.driver {
            self.driver = other_value;
        }
        if let Some(other_value) = other.pool {
            self.pool = other_value;
        }
        if let Some(other_value) = other.request {
            self.request = other_value;
        }
        self.share_id = other.share_id;
        crate::OptionableConvert::merge(&mut self.tolerations, other.tolerations)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::resource::v1::DeviceRequestAllocationResult,
> for DeviceRequestAllocationResultAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1::DeviceRequestAllocationResult,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1::DeviceRequestAllocationResult,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1::DeviceRequestAllocationResult,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
