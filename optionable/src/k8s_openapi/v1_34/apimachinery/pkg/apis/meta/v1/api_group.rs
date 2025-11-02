#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct APIGroupAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_version: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_address_by_client_cidrs: <Option<
        std::vec::Vec<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<
        <std::vec::Vec<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroup {
    type Optioned = APIGroupAc;
}
#[automatically_derived]
impl crate::Optionable for APIGroupAc {
    type Optioned = APIGroupAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroup {
    fn into_optioned(self) -> APIGroupAc {
        APIGroupAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            preferred_version: crate::OptionableConvert::into_optioned(
                self.preferred_version,
            ),
            server_address_by_client_cidrs: crate::OptionableConvert::into_optioned(
                self.server_address_by_client_cidrs,
            ),
            versions: Some(crate::OptionableConvert::into_optioned(self.versions)),
        }
    }
    fn try_from_optioned(value: APIGroupAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            preferred_version: crate::OptionableConvert::try_from_optioned(
                value.preferred_version,
            )?,
            server_address_by_client_cidrs: crate::OptionableConvert::try_from_optioned(
                value.server_address_by_client_cidrs,
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
    fn merge(&mut self, other: APIGroupAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.preferred_version,
            other.preferred_version,
        )?;
        crate::OptionableConvert::merge(
            &mut self.server_address_by_client_cidrs,
            other.server_address_by_client_cidrs,
        )?;
        if let Some(other_value) = other.versions {
            crate::OptionableConvert::merge(&mut self.versions, other_value)?;
        }
        Ok(())
    }
}
