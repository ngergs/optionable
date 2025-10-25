pub struct APIGroupOpt {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub preferred_version: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery,
    > as crate::Optionable>::Optioned,
    pub server_address_by_client_cidrs: <Option<
        std::vec::Vec<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR,
        >,
    > as crate::Optionable>::Optioned,
    pub versions: Option<
        <std::vec::Vec<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroup {
    type Optioned = APIGroupOpt;
}
#[automatically_derived]
impl crate::Optionable for APIGroupOpt {
    type Optioned = APIGroupOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroup {
    fn into_optioned(self) -> APIGroupOpt {
        APIGroupOpt {
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            preferred_version: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery,
            > as crate::OptionableConvert>::into_optioned(self.preferred_version),
            server_address_by_client_cidrs: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR,
                >,
            > as crate::OptionableConvert>::into_optioned(
                self.server_address_by_client_cidrs,
            ),
            versions: Some(
                <std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery,
                > as crate::OptionableConvert>::into_optioned(self.versions),
            ),
        }
    }
    fn try_from_optioned(value: APIGroupOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            preferred_version: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery,
            > as crate::OptionableConvert>::try_from_optioned(value.preferred_version)?,
            server_address_by_client_cidrs: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR,
                >,
            > as crate::OptionableConvert>::try_from_optioned(
                value.server_address_by_client_cidrs,
            )?,
            versions: <std::vec::Vec<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .versions
                    .ok_or(crate::optionable::Error {
                        missing_field: "versions",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: APIGroupOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery,
        > as crate::OptionableConvert>::merge(
            &mut self.preferred_version,
            other.preferred_version,
        )?;
        <Option<
            std::vec::Vec<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.server_address_by_client_cidrs,
            other.server_address_by_client_cidrs,
        )?;
        if let Some(other_value) = other.versions {
            <std::vec::Vec<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery,
            > as crate::OptionableConvert>::merge(&mut self.versions, other_value)?;
        }
        Ok(())
    }
}
