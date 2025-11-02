#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DeviceAllocationResultAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceAllocationConfiguration>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceRequestAllocationResult>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::DeviceAllocationResult {
    type Optioned = DeviceAllocationResultAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceAllocationResultAc {
    type Optioned = DeviceAllocationResultAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1::DeviceAllocationResult {
    fn into_optioned(self) -> DeviceAllocationResultAc {
        DeviceAllocationResultAc {
            config: crate::OptionableConvert::into_optioned(self.config),
            results: crate::OptionableConvert::into_optioned(self.results),
        }
    }
    fn try_from_optioned(
        value: DeviceAllocationResultAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            config: crate::OptionableConvert::try_from_optioned(value.config)?,
            results: crate::OptionableConvert::try_from_optioned(value.results)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceAllocationResultAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.config, other.config)?;
        crate::OptionableConvert::merge(&mut self.results, other.results)?;
        Ok(())
    }
}
