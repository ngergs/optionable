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
    /// Controller is the name of the DRA driver which handled the allocation. That driver is also responsible for deallocating the claim. It is empty when the claim can be deallocated without involving a driver.
    ///
    /// A driver may allocate devices provided by other drivers, so this driver name here can be different from the driver names listed for the results.
    ///
    /// This is an alpha field and requires enabling the DRAControlPlaneController feature gate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<std::string::String>,
    /// Devices is the result of allocating devices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<
        <::k8s_openapi027::api::resource::v1alpha3::DeviceAllocationResult as crate::Optionable>::Optioned,
    >,
    /// NodeSelector defines where the allocated resources are available. If unset, they are available everywhere.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<
        <::k8s_openapi027::api::core::v1::NodeSelector as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1alpha3::AllocationResult {
    type Optioned = AllocationResultAc;
}
#[automatically_derived]
impl crate::Optionable for AllocationResultAc {
    type Optioned = AllocationResultAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1alpha3::AllocationResult {
    fn into_optioned(self) -> AllocationResultAc {
        AllocationResultAc {
            controller: self.controller,
            devices: crate::OptionableConvert::into_optioned(self.devices),
            node_selector: crate::OptionableConvert::into_optioned(self.node_selector),
        }
    }
    fn try_from_optioned(value: AllocationResultAc) -> Result<Self, crate::Error> {
        Ok(Self {
            controller: value.controller,
            devices: crate::OptionableConvert::try_from_optioned(value.devices)?,
            node_selector: crate::OptionableConvert::try_from_optioned(
                value.node_selector,
            )?,
        })
    }
    fn merge(&mut self, other: AllocationResultAc) -> Result<(), crate::Error> {
        if self.controller.is_none() {
            self.controller = crate::OptionableConvert::try_from_optioned(
                other.controller,
            )?;
        } else if let Some(self_value) = self.controller.as_mut()
            && let Some(other_value) = other.controller
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
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1alpha3::AllocationResult>
for AllocationResultAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::AllocationResult,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1alpha3::AllocationResult,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::AllocationResult,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for AllocationResultAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.controller, other.controller);
        k8s_openapi027::DeepMerge::merge_from(&mut self.devices, other.devices);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.node_selector,
            other.node_selector,
        );
    }
}
