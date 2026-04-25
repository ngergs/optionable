#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// TCPSocketAction describes an action based on opening a socket
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TCPSocketActionAc {
    /// Optional: Host name to connect to, defaults to the pod IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<std::string::String>,
    /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
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
            host: self.host,
            port: Some(crate::OptionableConvert::into_optioned(self.port)),
        }
    }
    fn try_from_optioned(value: TCPSocketActionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            host: value.host,
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
        if self.host.is_none() {
            self.host = crate::OptionableConvert::try_from_optioned(other.host)?;
        } else if let Some(self_value) = self.host.as_mut()
            && let Some(other_value) = other.host
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
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
impl k8s_openapi027::DeepMerge for TCPSocketActionAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.host, other.host);
        k8s_openapi027::DeepMerge::merge_from(&mut self.port, other.port);
    }
}
