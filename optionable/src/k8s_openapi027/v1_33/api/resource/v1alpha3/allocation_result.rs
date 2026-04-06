#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AllocationResultAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: <Option<
        ::k8s_openapi027::api::resource::v1alpha3::DeviceAllocationResult,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_selector: <Option<
        ::k8s_openapi027::api::core::v1::NodeSelector,
    > as crate::Optionable>::Optioned,
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
            devices: crate::OptionableConvert::into_optioned(self.devices),
            node_selector: crate::OptionableConvert::into_optioned(self.node_selector),
        }
    }
    fn try_from_optioned(value: AllocationResultAc) -> Result<Self, crate::Error> {
        Ok(Self {
            devices: crate::OptionableConvert::try_from_optioned(value.devices)?,
            node_selector: crate::OptionableConvert::try_from_optioned(
                value.node_selector,
            )?,
        })
    }
    fn merge(&mut self, other: AllocationResultAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.devices, other.devices)?;
        crate::OptionableConvert::merge(&mut self.node_selector, other.node_selector)?;
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
