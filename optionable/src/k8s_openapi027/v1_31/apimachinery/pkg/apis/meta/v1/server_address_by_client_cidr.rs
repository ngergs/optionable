#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ServerAddressByClientCIDR helps the client to determine the server address that they should use, depending on the clientCIDR that they match.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServerAddressByClientCIDRAc {
    /// The CIDR with which clients can match their IP to figure out the server address that they should use.
    #[serde(rename = "clientCIDR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_cidr: Option<std::string::String>,
    /// Address of this server, suitable for a client that matches the above CIDR. This can be a hostname, hostname:port, IP or IP:port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_address: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR {
    type Optioned = ServerAddressByClientCIDRAc;
}
#[automatically_derived]
impl crate::Optionable for ServerAddressByClientCIDRAc {
    type Optioned = ServerAddressByClientCIDRAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR {
    fn into_optioned(self) -> ServerAddressByClientCIDRAc {
        ServerAddressByClientCIDRAc {
            client_cidr: Some(self.client_cidr),
            server_address: Some(self.server_address),
        }
    }
    fn try_from_optioned(
        value: ServerAddressByClientCIDRAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            client_cidr: value
                .client_cidr
                .ok_or(crate::Error {
                    missing_field: "client_cidr",
                })?,
            server_address: value
                .server_address
                .ok_or(crate::Error {
                    missing_field: "server_address",
                })?,
        })
    }
    fn merge(&mut self, other: ServerAddressByClientCIDRAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.client_cidr {
            self.client_cidr = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.server_address {
            self.server_address = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR,
> for ServerAddressByClientCIDRAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
