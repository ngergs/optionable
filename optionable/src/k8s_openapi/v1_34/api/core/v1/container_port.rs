pub struct ContainerPortAc {
    pub container_port: Option<i32>,
    pub host_ip: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub host_port: <Option<i32> as crate::Optionable>::Optioned,
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub protocol: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ContainerPort {
    type Optioned = ContainerPortAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerPortAc {
    type Optioned = ContainerPortAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ContainerPort {
    fn into_optioned(self) -> ContainerPortAc {
        ContainerPortAc {
            container_port: Some(self.container_port),
            host_ip: crate::OptionableConvert::into_optioned(self.host_ip),
            host_port: crate::OptionableConvert::into_optioned(self.host_port),
            name: crate::OptionableConvert::into_optioned(self.name),
            protocol: crate::OptionableConvert::into_optioned(self.protocol),
        }
    }
    fn try_from_optioned(
        value: ContainerPortAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            container_port: value
                .container_port
                .ok_or(crate::optionable::Error {
                    missing_field: "container_port",
                })?,
            host_ip: crate::OptionableConvert::try_from_optioned(value.host_ip)?,
            host_port: crate::OptionableConvert::try_from_optioned(value.host_port)?,
            name: crate::OptionableConvert::try_from_optioned(value.name)?,
            protocol: crate::OptionableConvert::try_from_optioned(value.protocol)?,
        })
    }
    fn merge(&mut self, other: ContainerPortAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.container_port {
            self.container_port = other_value;
        }
        crate::OptionableConvert::merge(&mut self.host_ip, other.host_ip)?;
        crate::OptionableConvert::merge(&mut self.host_port, other.host_port)?;
        crate::OptionableConvert::merge(&mut self.name, other.name)?;
        crate::OptionableConvert::merge(&mut self.protocol, other.protocol)?;
        Ok(())
    }
}
