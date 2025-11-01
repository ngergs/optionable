pub struct TCPSocketActionAc {
    pub host: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub port: Option<
        <::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::TCPSocketAction {
    type Optioned = TCPSocketActionAc;
}
#[automatically_derived]
impl crate::Optionable for TCPSocketActionAc {
    type Optioned = TCPSocketActionAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::TCPSocketAction {
    fn into_optioned(self) -> TCPSocketActionAc {
        TCPSocketActionAc {
            host: crate::OptionableConvert::into_optioned(self.host),
            port: Some(crate::OptionableConvert::into_optioned(self.port)),
        }
    }
    fn try_from_optioned(
        value: TCPSocketActionAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            host: crate::OptionableConvert::try_from_optioned(value.host)?,
            port: crate::OptionableConvert::try_from_optioned(
                value
                    .port
                    .ok_or(crate::optionable::Error {
                        missing_field: "port",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: TCPSocketActionAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.host, other.host)?;
        if let Some(other_value) = other.port {
            crate::OptionableConvert::merge(&mut self.port, other_value)?;
        }
        Ok(())
    }
}
