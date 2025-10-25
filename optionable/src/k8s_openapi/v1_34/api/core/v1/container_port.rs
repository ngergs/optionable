pub struct ContainerPortOpt {
    pub container_port: Option<i32>,
    pub host_ip: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub host_port: <Option<i32> as crate::Optionable>::Optioned,
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub protocol: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::container_port::ContainerPort {
    type Optioned = ContainerPortOpt;
}
#[automatically_derived]
impl crate::Optionable for ContainerPortOpt {
    type Optioned = ContainerPortOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::container_port::ContainerPort {
    fn into_optioned(self) -> ContainerPortOpt {
        ContainerPortOpt {
            container_port: Some(self.container_port),
            host_ip: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.host_ip),
            host_port: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.host_port),
            name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.name),
            protocol: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.protocol),
        }
    }
    fn try_from_optioned(
        value: ContainerPortOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            container_port: value
                .container_port
                .ok_or(crate::optionable::Error {
                    missing_field: "container_port",
                })?,
            host_ip: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.host_ip)?,
            host_port: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.host_port)?,
            name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.name)?,
            protocol: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.protocol)?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerPortOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.container_port {
            self.container_port = other_value;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.host_ip, other.host_ip)?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(&mut self.host_port, other.host_port)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.name, other.name)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.protocol, other.protocol)?;
        Ok(())
    }
}
