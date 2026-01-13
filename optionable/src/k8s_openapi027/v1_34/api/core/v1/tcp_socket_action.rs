#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TCPSocketActionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<
        <::k8s_openapi027::apimachinery::pkg::util::intstr::IntOrString as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::TCPSocketAction {
    type Optioned = TCPSocketActionAc;
}
#[automatically_derived]
impl crate::Optionable for TCPSocketActionAc {
    type Optioned = TCPSocketActionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::TCPSocketAction {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::TCPSocketAction>
for TCPSocketActionAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::TCPSocketAction) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::TCPSocketAction, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::TCPSocketAction,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
