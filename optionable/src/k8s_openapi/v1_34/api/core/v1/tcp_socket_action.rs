pub struct TCPSocketActionOpt {
    pub host: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub port: Option<
        <::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::TCPSocketAction {
    type Optioned = TCPSocketActionOpt;
}
#[automatically_derived]
impl crate::Optionable for TCPSocketActionOpt {
    type Optioned = TCPSocketActionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::TCPSocketAction {
    fn into_optioned(self) -> TCPSocketActionOpt {
        TCPSocketActionOpt {
            host: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.host),
            port: Some(
                <::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString as crate::OptionableConvert>::into_optioned(
                    self.port,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: TCPSocketActionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            host: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.host)?,
            port: <::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString as crate::OptionableConvert>::try_from_optioned(
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
        other: TCPSocketActionOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.host, other.host)?;
        if let Some(other_value) = other.port {
            <::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString as crate::OptionableConvert>::merge(
                &mut self.port,
                other_value,
            )?;
        }
        Ok(())
    }
}
