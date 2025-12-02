#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct ServerAddressByClientCIDRAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_cidr: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_address: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR {
    type Optioned = ServerAddressByClientCIDRAc;
}
#[automatically_derived]
impl crate::Optionable for ServerAddressByClientCIDRAc {
    type Optioned = ServerAddressByClientCIDRAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR {
    fn into_optioned(self) -> ServerAddressByClientCIDRAc {
        ServerAddressByClientCIDRAc {
            client_cidr: Some(crate::OptionableConvert::into_optioned(self.client_cidr)),
            server_address: Some(
                crate::OptionableConvert::into_optioned(self.server_address),
            ),
        }
    }
    fn try_from_optioned(
        value: ServerAddressByClientCIDRAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            client_cidr: crate::OptionableConvert::try_from_optioned(
                value
                    .client_cidr
                    .ok_or(crate::Error {
                        missing_field: "client_cidr",
                    })?,
            )?,
            server_address: crate::OptionableConvert::try_from_optioned(
                value
                    .server_address
                    .ok_or(crate::Error {
                        missing_field: "server_address",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: ServerAddressByClientCIDRAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.client_cidr {
            crate::OptionableConvert::merge(&mut self.client_cidr, other_value)?;
        }
        if let Some(other_value) = other.server_address {
            crate::OptionableConvert::merge(&mut self.server_address, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR,
> for ServerAddressByClientCIDRAc {
    fn from_optionable(
        value: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
