#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DeviceRequest is a request for devices required for a claim. This is typically a request for a single resource like a device, but can also ask for several identical devices.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceRequestAc {
    /// AdminAccess indicates that this is a claim for administrative access to the device(s). Claims with AdminAccess are expected to be used for monitoring or other management services for a device.  They ignore all ordinary claims to the device with respect to access modes and any resource allocations.
    ///
    /// This field can only be set when deviceClassName is set and no subrequests are specified in the firstAvailable list.
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
    /// This field can only be set when deviceClassName is set and no subrequests are specified in the firstAvailable list.
    ///
    /// More modes may get added in the future. Clients must refuse to handle requests with unknown modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_mode: Option<std::string::String>,
    /// Capacity define resource requirements against each capacity.
    ///
    /// If this field is unset and the device supports multiple allocations, the default value will be applied to each capacity according to requestPolicy. For the capacity that has no requestPolicy, default is the full capacity value.
    ///
    /// Applies to each device allocation. If Count \> 1, the request fails if there aren't enough devices that meet the requirements. If AllocationMode is set to All, the request fails if there are devices that otherwise match the request, and have this capacity, with a value \>= the requested amount, but which cannot be allocated to this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<
        <::k8s_openapi027::api::resource::v1beta1::CapacityRequirements as crate::Optionable>::Optioned,
    >,
    /// Count is used only when the count mode is "ExactCount". Must be greater than zero. If AllocationMode is ExactCount and this field is not specified, the default is one.
    ///
    /// This field can only be set when deviceClassName is set and no subrequests are specified in the firstAvailable list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// DeviceClassName references a specific DeviceClass, which can define additional configuration and selectors to be inherited by this request.
    ///
    /// A class is required if no subrequests are specified in the firstAvailable list and no class can be set if subrequests are specified in the firstAvailable list. Which classes are available depends on the cluster.
    ///
    /// Administrators may use this to restrict which devices may get requested by only installing classes with selectors for permitted devices. If users are free to request anything without restrictions, then administrators can create an empty DeviceClass for users to reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class_name: Option<std::string::String>,
    /// FirstAvailable contains subrequests, of which exactly one will be satisfied by the scheduler to satisfy this request. It tries to satisfy them in the order in which they are listed here. So if there are two entries in the list, the scheduler will only check the second one if it determines that the first one cannot be used.
    ///
    /// This field may only be set in the entries of DeviceClaim.Requests.
    ///
    /// DRA does not yet implement scoring, so the scheduler will select the first set of devices that satisfies all the requests in the claim. And if the requirements can be satisfied on more than one node, other scheduling features will determine which node is chosen. This means that the set of devices allocated to a claim might not be the optimal set available to the cluster. Scoring will be implemented later.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_available: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1beta1::DeviceSubRequest as crate::Optionable>::Optioned,
        >,
    >,
    /// Name can be used to reference this request in a pod.spec.containers\[\].resources.claims entry and in a constraint of the claim.
    ///
    /// Must be a DNS label and unique among all DeviceRequests in a ResourceClaim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// Selectors define criteria which must be satisfied by a specific device in order for that device to be considered for this request. All selectors must be satisfied for a device to be considered.
    ///
    /// This field can only be set when deviceClassName is set and no subrequests are specified in the firstAvailable list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1beta1::DeviceSelector as crate::Optionable>::Optioned,
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
    /// This field can only be set when deviceClassName is set and no subrequests are specified in the firstAvailable list.
    ///
    /// This is an alpha field and requires enabling the DRADeviceTaints feature gate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1beta1::DeviceToleration as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1beta1::DeviceRequest {
    type Optioned = DeviceRequestAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceRequestAc {
    type Optioned = DeviceRequestAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::resource::v1beta1::DeviceRequest {
    fn into_optioned(self) -> DeviceRequestAc {
        DeviceRequestAc {
            admin_access: self.admin_access,
            allocation_mode: self.allocation_mode,
            capacity: crate::OptionableConvert::into_optioned(self.capacity),
            count: self.count,
            device_class_name: self.device_class_name,
            first_available: crate::OptionableConvert::into_optioned(
                self.first_available,
            ),
            name: Some(self.name),
            selectors: crate::OptionableConvert::into_optioned(self.selectors),
            tolerations: crate::OptionableConvert::into_optioned(self.tolerations),
        }
    }
    fn try_from_optioned(value: DeviceRequestAc) -> Result<Self, crate::Error> {
        Ok(Self {
            admin_access: value.admin_access,
            allocation_mode: value.allocation_mode,
            capacity: crate::OptionableConvert::try_from_optioned(value.capacity)?,
            count: value.count,
            device_class_name: value.device_class_name,
            first_available: crate::OptionableConvert::try_from_optioned(
                value.first_available,
            )?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            selectors: crate::OptionableConvert::try_from_optioned(value.selectors)?,
            tolerations: crate::OptionableConvert::try_from_optioned(value.tolerations)?,
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
        if self.capacity.is_none() {
            self.capacity = crate::OptionableConvert::try_from_optioned(other.capacity)?;
        } else if let Some(self_value) = self.capacity.as_mut()
            && let Some(other_value) = other.capacity
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
        if self.device_class_name.is_none() {
            self.device_class_name = crate::OptionableConvert::try_from_optioned(
                other.device_class_name,
            )?;
        } else if let Some(self_value) = self.device_class_name.as_mut()
            && let Some(other_value) = other.device_class_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.first_available.is_none() {
            self.first_available = crate::OptionableConvert::try_from_optioned(
                other.first_available,
            )?;
        } else if let Some(self_value) = self.first_available.as_mut()
            && let Some(other_value) = other.first_available
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
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
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.tolerations.is_none() {
            self.tolerations = crate::OptionableConvert::try_from_optioned(
                other.tolerations,
            )?;
        } else if let Some(self_value) = self.tolerations.as_mut()
            && let Some(other_value) = other.tolerations
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1beta1::DeviceRequest>
for DeviceRequestAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta1::DeviceRequest,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1beta1::DeviceRequest, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta1::DeviceRequest,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for DeviceRequestAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.admin_access,
            other.admin_access,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.allocation_mode,
            other.allocation_mode,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.capacity, other.capacity);
        k8s_openapi027::DeepMerge::merge_from(&mut self.count, other.count);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.device_class_name,
            other.device_class_name,
        );
        self.first_available = other.first_available;
        k8s_openapi027::DeepMerge::merge_from(&mut self.name, other.name);
        self.selectors = other.selectors;
        self.tolerations = other.tolerations;
    }
}
