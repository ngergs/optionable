#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DeviceSubRequest describes a request for device provided in the claim.spec.devices.requests\[\].firstAvailable array. Each is typically a request for a single resource like a device, but can also ask for several identical devices.
///
/// DeviceSubRequest is similar to Request, but doesn't expose the AdminAccess or FirstAvailable fields, as those can only be set on the top-level request. AdminAccess is not supported for requests with a prioritized list, and recursive FirstAvailable fields are not supported.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceSubRequestAc {
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
    /// If AllocationMode is not specified, the default mode is ExactCount. If the mode is ExactCount and count is not specified, the default count is one. Any other requests must specify this field.
    ///
    /// More modes may get added in the future. Clients must refuse to handle requests with unknown modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_mode: Option<std::string::String>,
    /// Count is used only when the count mode is "ExactCount". Must be greater than zero. If AllocationMode is ExactCount and this field is not specified, the default is one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// DeviceClassName references a specific DeviceClass, which can define additional configuration and selectors to be inherited by this subrequest.
    ///
    /// A class is required. Which classes are available depends on the cluster.
    ///
    /// Administrators may use this to restrict which devices may get requested by only installing classes with selectors for permitted devices. If users are free to request anything without restrictions, then administrators can create an empty DeviceClass for users to reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class_name: Option<std::string::String>,
    /// Name can be used to reference this subrequest in the list of constraints or the list of configurations for the claim. References must use the format \<main request\>/\<subrequest\>.
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
            <::k8s_openapi027::api::resource::v1alpha3::DeviceToleration as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1alpha3::DeviceSubRequest {
    type Optioned = DeviceSubRequestAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceSubRequestAc {
    type Optioned = DeviceSubRequestAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1alpha3::DeviceSubRequest {
    fn into_optioned(self) -> DeviceSubRequestAc {
        DeviceSubRequestAc {
            allocation_mode: self.allocation_mode,
            count: self.count,
            device_class_name: Some(self.device_class_name),
            name: Some(self.name),
            selectors: crate::OptionableConvert::into_optioned(self.selectors),
            tolerations: crate::OptionableConvert::into_optioned(self.tolerations),
        }
    }
    fn try_from_optioned(value: DeviceSubRequestAc) -> Result<Self, crate::Error> {
        Ok(Self {
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
            tolerations: crate::OptionableConvert::try_from_optioned(value.tolerations)?,
        })
    }
    fn merge(&mut self, other: DeviceSubRequestAc) -> Result<(), crate::Error> {
        self.allocation_mode = other.allocation_mode;
        self.count = other.count;
        if let Some(other_value) = other.device_class_name {
            self.device_class_name = other_value;
        }
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        crate::OptionableConvert::merge(&mut self.selectors, other.selectors)?;
        crate::OptionableConvert::merge(&mut self.tolerations, other.tolerations)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1alpha3::DeviceSubRequest>
for DeviceSubRequestAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::DeviceSubRequest,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1alpha3::DeviceSubRequest,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::DeviceSubRequest,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
