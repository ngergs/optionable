#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DeviceRequest is a request for devices required for a claim. This is typically a request for a single resource like a device, but can also ask for several identical devices.
///
/// A DeviceClassName is currently required. Clients must check that it is indeed set. It's absence indicates that something changed in a way that is not supported by the client yet, in which case it must refuse to handle the request.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceRequestAc {
    /// AdminAccess indicates that this is a claim for administrative access to the device(s). Claims with AdminAccess are expected to be used for monitoring or other management services for a device.  They ignore all ordinary claims to the device with respect to access modes and any resource allocations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_access: Option<bool>,
    /// AllocationMode and its related fields define how devices are allocated to satisfy this request. Supported values are:
    ///
    /// - ExactCount: This request is for a specific number of devices.
    ///   This is the default. The exact number is provided in the
    ///   count field.
    ///
    /// - All: This request is for all of the matching devices in a pool.
    ///   Allocation will fail if some devices are already allocated,
    ///   unless adminAccess is requested.
    ///
    /// If AlloctionMode is not specified, the default mode is ExactCount. If the mode is ExactCount and count is not specified, the default count is one. Any other requests must specify this field.
    ///
    /// More modes may get added in the future. Clients must refuse to handle requests with unknown modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_mode: Option<std::string::String>,
    /// Count is used only when the count mode is "ExactCount". Must be greater than zero. If AllocationMode is ExactCount and this field is not specified, the default is one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// DeviceClassName references a specific DeviceClass, which can define additional configuration and selectors to be inherited by this request.
    ///
    /// A class is required. Which classes are available depends on the cluster.
    ///
    /// Administrators may use this to restrict which devices may get requested by only installing classes with selectors for permitted devices. If users are free to request anything without restrictions, then administrators can create an empty DeviceClass for users to reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class_name: Option<std::string::String>,
    /// Name can be used to reference this request in a pod.spec.containers\[\].resources.claims entry and in a constraint of the claim.
    ///
    /// Must be a DNS label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// Selectors define criteria which must be satisfied by a specific device in order for that device to be considered for this request. All selectors must be satisfied for a device to be considered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1alpha3::DeviceSelector as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1alpha3::DeviceRequest {
    type Optioned = DeviceRequestAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceRequestAc {
    type Optioned = DeviceRequestAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1alpha3::DeviceRequest {
    fn into_optioned(self) -> DeviceRequestAc {
        DeviceRequestAc {
            admin_access: self.admin_access,
            allocation_mode: self.allocation_mode,
            count: self.count,
            device_class_name: Some(self.device_class_name),
            name: Some(self.name),
            selectors: crate::OptionableConvert::into_optioned(self.selectors),
        }
    }
    fn try_from_optioned(value: DeviceRequestAc) -> Result<Self, crate::Error> {
        Ok(Self {
            admin_access: value.admin_access,
            allocation_mode: value.allocation_mode,
            count: value.count,
            device_class_name: value
                .device_class_name
                .ok_or(crate::Error {
                    missing_field: "device_class_name",
                })?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            selectors: crate::OptionableConvert::try_from_optioned(value.selectors)?,
        })
    }
    fn merge(&mut self, other: DeviceRequestAc) -> Result<(), crate::Error> {
        if self.admin_access.is_none() {
            self.admin_access = crate::OptionableConvert::try_from_optioned(
                other.admin_access,
            )?;
        } else if let Some(self_value) = self.admin_access.as_mut()
            && let Some(other_value) = other.admin_access
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.allocation_mode.is_none() {
            self.allocation_mode = crate::OptionableConvert::try_from_optioned(
                other.allocation_mode,
            )?;
        } else if let Some(self_value) = self.allocation_mode.as_mut()
            && let Some(other_value) = other.allocation_mode
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.count.is_none() {
            self.count = crate::OptionableConvert::try_from_optioned(other.count)?;
        } else if let Some(self_value) = self.count.as_mut()
            && let Some(other_value) = other.count
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.device_class_name {
            self.device_class_name = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if let Some(other_value) = other.name {
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.selectors.is_none() {
            self.selectors = crate::OptionableConvert::try_from_optioned(
                other.selectors,
            )?;
        } else if let Some(self_value) = self.selectors.as_mut()
            && let Some(other_value) = other.selectors
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1alpha3::DeviceRequest>
for DeviceRequestAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::DeviceRequest,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1alpha3::DeviceRequest, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::DeviceRequest,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
