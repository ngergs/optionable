#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// AllocationResult contains attributes of an allocated resource.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AllocationResultAc {
    /// AllocationTimestamp stores the time when the resources were allocated. This field is not guaranteed to be set, in which case that time is unknown.
    ///
    /// This is an alpha field and requires enabling the DRADeviceBindingConditions and DRAResourceClaimDeviceStatus feature gate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_timestamp: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// Devices is the result of allocating devices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<
        <::k8s_openapi027::api::resource::v1::DeviceAllocationResult as crate::Optionable>::Optioned,
    >,
    /// NodeSelector defines where the allocated resources are available. If unset, they are available everywhere.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<
        <::k8s_openapi027::api::core::v1::NodeSelector as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1::AllocationResult {
    type Optioned = AllocationResultAc;
}
#[automatically_derived]
impl crate::Optionable for AllocationResultAc {
    type Optioned = AllocationResultAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::resource::v1::AllocationResult {
    fn into_optioned(self) -> AllocationResultAc {
        AllocationResultAc {
            allocation_timestamp: crate::OptionableConvert::into_optioned(
                self.allocation_timestamp,
            ),
            devices: crate::OptionableConvert::into_optioned(self.devices),
            node_selector: crate::OptionableConvert::into_optioned(self.node_selector),
        }
    }
    fn try_from_optioned(value: AllocationResultAc) -> Result<Self, crate::Error> {
        Ok(Self {
            allocation_timestamp: crate::OptionableConvert::try_from_optioned(
                value.allocation_timestamp,
            )?,
            devices: crate::OptionableConvert::try_from_optioned(value.devices)?,
            node_selector: crate::OptionableConvert::try_from_optioned(
                value.node_selector,
            )?,
        })
    }
    fn merge(&mut self, other: AllocationResultAc) -> Result<(), crate::Error> {
        if self.allocation_timestamp.is_none() {
            self.allocation_timestamp = crate::OptionableConvert::try_from_optioned(
                other.allocation_timestamp,
            )?;
        } else if let Some(self_value) = self.allocation_timestamp.as_mut()
            && let Some(other_value) = other.allocation_timestamp
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.devices.is_none() {
            self.devices = crate::OptionableConvert::try_from_optioned(other.devices)?;
        } else if let Some(self_value) = self.devices.as_mut()
            && let Some(other_value) = other.devices
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.node_selector.is_none() {
            self.node_selector = crate::OptionableConvert::try_from_optioned(
                other.node_selector,
            )?;
        } else if let Some(self_value) = self.node_selector.as_mut()
            && let Some(other_value) = other.node_selector
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1::AllocationResult>
for AllocationResultAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1::AllocationResult,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1::AllocationResult, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1::AllocationResult,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for AllocationResultAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.allocation_timestamp,
            other.allocation_timestamp,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.devices, other.devices);
        self.node_selector = other.node_selector;
    }
}
