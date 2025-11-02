#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct APIVersionsAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_address_by_client_cidrs: Option<
        <std::vec::Vec<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR,
        > as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIVersions {
    type Optioned = APIVersionsAc;
}
#[automatically_derived]
impl crate::Optionable for APIVersionsAc {
    type Optioned = APIVersionsAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIVersions {
    fn into_optioned(self) -> APIVersionsAc {
        APIVersionsAc {
            server_address_by_client_cidrs: Some(
                crate::OptionableConvert::into_optioned(
                    self.server_address_by_client_cidrs,
                ),
            ),
            versions: Some(crate::OptionableConvert::into_optioned(self.versions)),
        }
    }
    fn try_from_optioned(
        value: APIVersionsAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            server_address_by_client_cidrs: crate::OptionableConvert::try_from_optioned(
                value
                    .server_address_by_client_cidrs
                    .ok_or(crate::optionable::Error {
                        missing_field: "server_address_by_client_cidrs",
                    })?,
            )?,
            versions: crate::OptionableConvert::try_from_optioned(
                value
                    .versions
                    .ok_or(crate::optionable::Error {
                        missing_field: "versions",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: APIVersionsAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.server_address_by_client_cidrs {
            crate::OptionableConvert::merge(
                &mut self.server_address_by_client_cidrs,
                other_value,
            )?;
        }
        if let Some(other_value) = other.versions {
            crate::OptionableConvert::merge(&mut self.versions, other_value)?;
        }
        Ok(())
    }
}
impl k8s_openapi::Resource for APIVersionsAc {
    const API_VERSION: &'static str = "v1";
    const GROUP: &'static str = "";
    const KIND: &'static str = "APIVersions";
    const VERSION: &'static str = "v1";
    const URL_PATH_SEGMENT: &'static str = "";
    type Scope = k8s_openapi::ClusterResourceScope;
}
