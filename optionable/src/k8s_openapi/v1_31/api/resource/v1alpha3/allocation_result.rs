#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct AllocationResultAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: <Option<
        ::k8s_openapi::api::resource::v1alpha3::DeviceAllocationResult,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_selector: <Option<
        ::k8s_openapi::api::core::v1::NodeSelector,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha3::AllocationResult {
    type Optioned = AllocationResultAc;
}
#[automatically_derived]
impl crate::Optionable for AllocationResultAc {
    type Optioned = AllocationResultAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha3::AllocationResult {
    fn into_optioned(self) -> AllocationResultAc {
        AllocationResultAc {
            controller: crate::OptionableConvert::into_optioned(self.controller),
            devices: crate::OptionableConvert::into_optioned(self.devices),
            node_selector: crate::OptionableConvert::into_optioned(self.node_selector),
        }
    }
    fn try_from_optioned(value: AllocationResultAc) -> Result<Self, crate::Error> {
        Ok(Self {
            controller: crate::OptionableConvert::try_from_optioned(value.controller)?,
            devices: crate::OptionableConvert::try_from_optioned(value.devices)?,
            node_selector: crate::OptionableConvert::try_from_optioned(
                value.node_selector,
            )?,
        })
    }
    fn merge(&mut self, other: AllocationResultAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.controller, other.controller)?;
        crate::OptionableConvert::merge(&mut self.devices, other.devices)?;
        crate::OptionableConvert::merge(&mut self.node_selector, other.node_selector)?;
        Ok(())
    }
}
