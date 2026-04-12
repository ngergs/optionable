#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ExactDeviceRequest is a request for one or more identical devices.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExactDeviceRequestAc {
    /// AdminAccess indicates that this is a claim for administrative access to the device(s). Claims with AdminAccess are expected to be used for monitoring or other management services for a device.  They ignore all ordinary claims to the device with respect to access modes and any resource allocations.
    ///
    /// This is an alpha field and requires enabling the DRAAdminAccess feature gate. Admin access is disabled if this field is unset or set to false, otherwise it is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_access: Option<bool>,
    /// AllocationMode and its related fields define how devices are allocated to satisfy this request. Supported values are:
    ///
    /// - ExactCount: This request is for a specific number of devices.
    ///   This is the default. The exact number is provided in the
    ///   count field.
    ///
    /// - All: This request is for all of the matching devices in a pool.
    ///   At least one device must exist on the node for the allocation to succeed.
    ///   Allocation will fail if some devices are already allocated,
    ///   unless adminAccess is requested.
    ///
    /// If AllocationMode is not specified, the default mode is ExactCount. If the mode is ExactCount and count is not specified, the default count is one. Any other requests must specify this field.
    ///
    /// More modes may get added in the future. Clients must refuse to handle requests with unknown modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_mode: Option<std::string::String>,
    /// Count is used only when the count mode is "ExactCount". Must be greater than zero. If AllocationMode is ExactCount and this field is not specified, the default is one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// DeviceClassName references a specific DeviceClass, which can define additional configuration and selectors to be inherited by this request.
    ///
    /// A DeviceClassName is required.
    ///
    /// Administrators may use this to restrict which devices may get requested by only installing classes with selectors for permitted devices. If users are free to request anything without restrictions, then administrators can create an empty DeviceClass for users to reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class_name: Option<std::string::String>,
    /// Selectors define criteria which must be satisfied by a specific device in order for that device to be considered for this request. All selectors must be satisfied for a device to be considered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1beta2::DeviceSelector as crate::Optionable>::Optioned,
        >,
    >,
    /// If specified, the request's tolerations.
    ///
    /// Tolerations for NoSchedule are required to allocate a device which has a taint with that effect. The same applies to NoExecute.
    ///
    /// In addition, should any of the allocated devices get tainted with NoExecute after allocation and that effect is not tolerated, then all pods consuming the ResourceClaim get deleted to evict them. The scheduler will not let new pods reserve the claim while it has these tainted devices. Once all pods are evicted, the claim will get deallocated.
    ///
    /// The maximum number of tolerations is 16.
    ///
    /// This is an alpha field and requires enabling the DRADeviceTaints feature gate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1beta2::DeviceToleration as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1beta2::ExactDeviceRequest {
    type Optioned = ExactDeviceRequestAc;
}
#[automatically_derived]
impl crate::Optionable for ExactDeviceRequestAc {
    type Optioned = ExactDeviceRequestAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1beta2::ExactDeviceRequest {
    fn into_optioned(self) -> ExactDeviceRequestAc {
        ExactDeviceRequestAc {
            admin_access: self.admin_access,
            allocation_mode: self.allocation_mode,
            count: self.count,
            device_class_name: Some(self.device_class_name),
            selectors: crate::OptionableConvert::into_optioned(self.selectors),
            tolerations: crate::OptionableConvert::into_optioned(self.tolerations),
        }
    }
    fn try_from_optioned(value: ExactDeviceRequestAc) -> Result<Self, crate::Error> {
        Ok(Self {
            admin_access: value.admin_access,
            allocation_mode: value.allocation_mode,
            count: value.count,
            device_class_name: value
                .device_class_name
                .ok_or(crate::Error {
                    missing_field: "device_class_name",
                })?,
            selectors: crate::OptionableConvert::try_from_optioned(value.selectors)?,
            tolerations: crate::OptionableConvert::try_from_optioned(value.tolerations)?,
        })
    }
    fn merge(&mut self, other: ExactDeviceRequestAc) -> Result<(), crate::Error> {
        if self.admin_access.is_none() {
            self.admin_access = crate::OptionableConvert::try_from_optioned(
                other.admin_access,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.admin_access, other.admin_access)?;
        }
        if self.allocation_mode.is_none() {
            self.allocation_mode = crate::OptionableConvert::try_from_optioned(
                other.allocation_mode,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.allocation_mode,
                other.allocation_mode,
            )?;
        }
        if self.count.is_none() {
            self.count = crate::OptionableConvert::try_from_optioned(other.count)?;
        } else {
            crate::OptionableConvert::merge(&mut self.count, other.count)?;
        }
        if let Some(other_value) = other.device_class_name {
            self.device_class_name = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if self.selectors.is_none() {
            self.selectors = crate::OptionableConvert::try_from_optioned(
                other.selectors,
            )?;
        } else {
            self.selectors = crate::OptionableConvert::try_from_optioned(
                other.selectors,
            )?;
        }
        if self.tolerations.is_none() {
            self.tolerations = crate::OptionableConvert::try_from_optioned(
                other.tolerations,
            )?;
        } else {
            self.tolerations = crate::OptionableConvert::try_from_optioned(
                other.tolerations,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1beta2::ExactDeviceRequest>
for ExactDeviceRequestAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta2::ExactDeviceRequest,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1beta2::ExactDeviceRequest,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta2::ExactDeviceRequest,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
