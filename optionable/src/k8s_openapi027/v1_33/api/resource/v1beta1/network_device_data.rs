#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NetworkDeviceData provides network-related details for the allocated device. This information may be filled by drivers or other components to configure or identify the device within a network context.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NetworkDeviceDataAc {
    /// HardwareAddress represents the hardware address (e.g. MAC Address) of the device's network interface.
    ///
    /// Must not be longer than 128 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_address: Option<std::string::String>,
    /// InterfaceName specifies the name of the network interface associated with the allocated device. This might be the name of a physical or virtual network interface being configured in the pod.
    ///
    /// Must not be longer than 256 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_name: Option<std::string::String>,
    /// IPs lists the network addresses assigned to the device's network interface. This can include both IPv4 and IPv6 addresses. The IPs are in the CIDR notation, which includes both the address and the associated subnet mask. e.g.: "192.0.2.5/24" for IPv4 and "2001:db8::5/64" for IPv6.
    ///
    /// Must not contain more than 16 entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ips: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1beta1::NetworkDeviceData {
    type Optioned = NetworkDeviceDataAc;
}
#[automatically_derived]
impl crate::Optionable for NetworkDeviceDataAc {
    type Optioned = NetworkDeviceDataAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1beta1::NetworkDeviceData {
    fn into_optioned(self) -> NetworkDeviceDataAc {
        NetworkDeviceDataAc {
            hardware_address: self.hardware_address,
            interface_name: self.interface_name,
            ips: self.ips,
        }
    }
    fn try_from_optioned(value: NetworkDeviceDataAc) -> Result<Self, crate::Error> {
        Ok(Self {
            hardware_address: value.hardware_address,
            interface_name: value.interface_name,
            ips: value.ips,
        })
    }
    fn merge(&mut self, other: NetworkDeviceDataAc) -> Result<(), crate::Error> {
        if self.hardware_address.is_none() {
            self.hardware_address = other.hardware_address;
        }
        if let Some(other_value) = other.hardware_address {
            crate::OptionableConvert::merge(&mut self.hardware_address, other_value)?;
        }
        if self.interface_name.is_none() {
            self.interface_name = other.interface_name;
        }
        if let Some(other_value) = other.interface_name {
            crate::OptionableConvert::merge(&mut self.interface_name, other_value)?;
        }
        if self.ips.is_none() {
            self.ips = other.ips;
        }
        if let Some(other_value) = other.ips {
            self.ips = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1beta1::NetworkDeviceData>
for NetworkDeviceDataAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta1::NetworkDeviceData,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1beta1::NetworkDeviceData,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta1::NetworkDeviceData,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
