#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct NetworkDeviceDataAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_address: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ips: <Option<std::vec::Vec<std::string::String>> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta2::NetworkDeviceData {
    type Optioned = NetworkDeviceDataAc;
}
#[automatically_derived]
impl crate::Optionable for NetworkDeviceDataAc {
    type Optioned = NetworkDeviceDataAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta2::NetworkDeviceData {
    fn into_optioned(self) -> NetworkDeviceDataAc {
        NetworkDeviceDataAc {
            hardware_address: crate::OptionableConvert::into_optioned(
                self.hardware_address,
            ),
            interface_name: crate::OptionableConvert::into_optioned(self.interface_name),
            ips: crate::OptionableConvert::into_optioned(self.ips),
        }
    }
    fn try_from_optioned(value: NetworkDeviceDataAc) -> Result<Self, crate::Error> {
        Ok(Self {
            hardware_address: crate::OptionableConvert::try_from_optioned(
                value.hardware_address,
            )?,
            interface_name: crate::OptionableConvert::try_from_optioned(
                value.interface_name,
            )?,
            ips: crate::OptionableConvert::try_from_optioned(value.ips)?,
        })
    }
    fn merge(&mut self, other: NetworkDeviceDataAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.hardware_address,
            other.hardware_address,
        )?;
        crate::OptionableConvert::merge(&mut self.interface_name, other.interface_name)?;
        crate::OptionableConvert::merge(&mut self.ips, other.ips)?;
        Ok(())
    }
}
