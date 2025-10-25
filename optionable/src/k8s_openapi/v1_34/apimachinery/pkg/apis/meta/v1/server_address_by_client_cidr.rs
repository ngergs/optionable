pub struct ServerAddressByClientCIDROpt {
    pub client_cidr: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub server_address: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR {
    type Optioned = ServerAddressByClientCIDROpt;
}
#[automatically_derived]
impl crate::Optionable for ServerAddressByClientCIDROpt {
    type Optioned = ServerAddressByClientCIDROpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR {
    fn into_optioned(self) -> ServerAddressByClientCIDROpt {
        ServerAddressByClientCIDROpt {
            client_cidr: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.client_cidr,
                ),
            ),
            server_address: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.server_address,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: ServerAddressByClientCIDROpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            client_cidr: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .client_cidr
                    .ok_or(crate::optionable::Error {
                        missing_field: "client_cidr",
                    })?,
            )?,
            server_address: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .server_address
                    .ok_or(crate::optionable::Error {
                        missing_field: "server_address",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ServerAddressByClientCIDROpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.client_cidr {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.client_cidr,
                other_value,
            )?;
        }
        if let Some(other_value) = other.server_address {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.server_address,
                other_value,
            )?;
        }
        Ok(())
    }
}
