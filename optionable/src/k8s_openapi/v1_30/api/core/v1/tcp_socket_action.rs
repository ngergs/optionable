#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct TCPSocketActionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    fn try_from_optioned(value: TCPSocketActionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            host: crate::OptionableConvert::try_from_optioned(value.host)?,
            port: crate::OptionableConvert::try_from_optioned(
                value
                    .port
                    .ok_or(crate::Error {
                        missing_field: "port",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: TCPSocketActionAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.host, other.host)?;
        if let Some(other_value) = other.port {
            crate::OptionableConvert::merge(&mut self.port, other_value)?;
        }
        Ok(())
    }
}
