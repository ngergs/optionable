#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClaimConfigurationAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opaque: <Option<
        ::k8s_openapi::api::resource::v1beta2::OpaqueDeviceConfiguration,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1beta2::DeviceClaimConfiguration {
    type Optioned = DeviceClaimConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceClaimConfigurationAc {
    type Optioned = DeviceClaimConfigurationAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta2::DeviceClaimConfiguration {
    fn into_optioned(self) -> DeviceClaimConfigurationAc {
        DeviceClaimConfigurationAc {
            opaque: crate::OptionableConvert::into_optioned(self.opaque),
            requests: crate::OptionableConvert::into_optioned(self.requests),
        }
    }
    fn try_from_optioned(
        value: DeviceClaimConfigurationAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            opaque: crate::OptionableConvert::try_from_optioned(value.opaque)?,
            requests: crate::OptionableConvert::try_from_optioned(value.requests)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceClaimConfigurationAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.opaque, other.opaque)?;
        crate::OptionableConvert::merge(&mut self.requests, other.requests)?;
        Ok(())
    }
}
