pub struct EndpointPortAc {
    pub app_protocol: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub port: Option<i32>,
    pub protocol: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::EndpointPort {
    type Optioned = EndpointPortAc;
}
#[automatically_derived]
impl crate::Optionable for EndpointPortAc {
    type Optioned = EndpointPortAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::EndpointPort {
    fn into_optioned(self) -> EndpointPortAc {
        EndpointPortAc {
            app_protocol: crate::OptionableConvert::into_optioned(self.app_protocol),
            name: crate::OptionableConvert::into_optioned(self.name),
            port: Some(self.port),
            protocol: crate::OptionableConvert::into_optioned(self.protocol),
        }
    }
    fn try_from_optioned(
        value: EndpointPortAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            app_protocol: crate::OptionableConvert::try_from_optioned(
                value.app_protocol,
            )?,
            name: crate::OptionableConvert::try_from_optioned(value.name)?,
            port: value
                .port
                .ok_or(crate::optionable::Error {
                    missing_field: "port",
                })?,
            protocol: crate::OptionableConvert::try_from_optioned(value.protocol)?,
        })
    }
    fn merge(&mut self, other: EndpointPortAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.app_protocol, other.app_protocol)?;
        crate::OptionableConvert::merge(&mut self.name, other.name)?;
        if let Some(other_value) = other.port {
            self.port = other_value;
        }
        crate::OptionableConvert::merge(&mut self.protocol, other.protocol)?;
        Ok(())
    }
}
