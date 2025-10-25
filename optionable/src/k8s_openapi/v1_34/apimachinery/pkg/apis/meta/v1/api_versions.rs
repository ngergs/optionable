pub struct APIVersionsOpt {
    pub server_address_by_client_cidrs: Option<
        <std::vec::Vec<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR,
        > as crate::Optionable>::Optioned,
    >,
    pub versions: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::api_versions::APIVersions {
    type Optioned = APIVersionsOpt;
}
#[automatically_derived]
impl crate::Optionable for APIVersionsOpt {
    type Optioned = APIVersionsOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::api_versions::APIVersions {
    fn into_optioned(self) -> APIVersionsOpt {
        APIVersionsOpt {
            server_address_by_client_cidrs: Some(
                <std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR,
                > as crate::OptionableConvert>::into_optioned(
                    self.server_address_by_client_cidrs,
                ),
            ),
            versions: Some(
                <std::vec::Vec<
                    std::string::String,
                > as crate::OptionableConvert>::into_optioned(self.versions),
            ),
        }
    }
    fn try_from_optioned(
        value: APIVersionsOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            server_address_by_client_cidrs: <std::vec::Vec<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .server_address_by_client_cidrs
                    .ok_or(crate::optionable::Error {
                        missing_field: "server_address_by_client_cidrs",
                    })?,
            )?,
            versions: <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .versions
                    .ok_or(crate::optionable::Error {
                        missing_field: "versions",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: APIVersionsOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.server_address_by_client_cidrs {
            <std::vec::Vec<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR,
            > as crate::OptionableConvert>::merge(
                &mut self.server_address_by_client_cidrs,
                other_value,
            )?;
        }
        if let Some(other_value) = other.versions {
            <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::merge(&mut self.versions, other_value)?;
        }
        Ok(())
    }
}
