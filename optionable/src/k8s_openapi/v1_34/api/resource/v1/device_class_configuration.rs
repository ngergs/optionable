#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClassConfigurationAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opaque: <Option<
        ::k8s_openapi::api::resource::v1::OpaqueDeviceConfiguration,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::DeviceClassConfiguration {
    type Optioned = DeviceClassConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceClassConfigurationAc {
    type Optioned = DeviceClassConfigurationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1::DeviceClassConfiguration {
    fn into_optioned(self) -> DeviceClassConfigurationAc {
        DeviceClassConfigurationAc {
            opaque: crate::OptionableConvert::into_optioned(self.opaque),
        }
    }
    fn try_from_optioned(
        value: DeviceClassConfigurationAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            opaque: crate::OptionableConvert::try_from_optioned(value.opaque)?,
        })
    }
    fn merge(&mut self, other: DeviceClassConfigurationAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.opaque, other.opaque)?;
        Ok(())
    }
}
