pub struct NetworkDeviceDataOpt {
    pub hardware_address: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub interface_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub ips: <Option<std::vec::Vec<std::string::String>> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta2::NetworkDeviceData {
    type Optioned = NetworkDeviceDataOpt;
}
#[automatically_derived]
impl crate::Optionable for NetworkDeviceDataOpt {
    type Optioned = NetworkDeviceDataOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta2::NetworkDeviceData {
    fn into_optioned(self) -> NetworkDeviceDataOpt {
        NetworkDeviceDataOpt {
            hardware_address: crate::OptionableConvert::into_optioned(
                self.hardware_address,
            ),
            interface_name: crate::OptionableConvert::into_optioned(self.interface_name),
            ips: crate::OptionableConvert::into_optioned(self.ips),
        }
    }
    fn try_from_optioned(
        value: NetworkDeviceDataOpt,
    ) -> Result<Self, crate::optionable::Error> {
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
    fn merge(
        &mut self,
        other: NetworkDeviceDataOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.hardware_address,
            other.hardware_address,
        )?;
        crate::OptionableConvert::merge(&mut self.interface_name, other.interface_name)?;
        crate::OptionableConvert::merge(&mut self.ips, other.ips)?;
        Ok(())
    }
}
