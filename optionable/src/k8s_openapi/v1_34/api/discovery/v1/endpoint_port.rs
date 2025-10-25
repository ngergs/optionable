pub struct EndpointPortOpt {
    pub app_protocol: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub port: <Option<i32> as crate::Optionable>::Optioned,
    pub protocol: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::discovery::v1::endpoint_port::EndpointPort {
    type Optioned = EndpointPortOpt;
}
#[automatically_derived]
impl crate::Optionable for EndpointPortOpt {
    type Optioned = EndpointPortOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::discovery::v1::endpoint_port::EndpointPort {
    fn into_optioned(self) -> EndpointPortOpt {
        EndpointPortOpt {
            app_protocol: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.app_protocol),
            name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.name),
            port: <Option<i32> as crate::OptionableConvert>::into_optioned(self.port),
            protocol: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.protocol),
        }
    }
    fn try_from_optioned(
        value: EndpointPortOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            app_protocol: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.app_protocol)?,
            name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.name)?,
            port: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.port)?,
            protocol: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.protocol)?,
        })
    }
    fn merge(&mut self, other: EndpointPortOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.app_protocol,
            other.app_protocol,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.name, other.name)?;
        <Option<i32> as crate::OptionableConvert>::merge(&mut self.port, other.port)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.protocol, other.protocol)?;
        Ok(())
    }
}
