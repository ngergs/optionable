#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct APIVersionsAc {
    #[serde(rename = "serverAddressByClientCIDRs")]
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
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_api_version",
        deserialize_with = "crate::k8s_openapi::deserialize_api_version"
    )]
    pub api_version: std::marker::PhantomData<Self>,
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_kind",
        deserialize_with = "crate::k8s_openapi::deserialize_kind"
    )]
    pub kind: std::marker::PhantomData<Self>,
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
#[cfg(feature = "k8s_openapi_convert")]
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
            api_version: Default::default(),
            kind: Default::default(),
        }
    }
    fn try_from_optioned(value: APIVersionsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            server_address_by_client_cidrs: crate::OptionableConvert::try_from_optioned(
                value
                    .server_address_by_client_cidrs
                    .ok_or(crate::Error {
                        missing_field: "server_address_by_client_cidrs",
                    })?,
            )?,
            versions: crate::OptionableConvert::try_from_optioned(
                value
                    .versions
                    .ok_or(crate::Error {
                        missing_field: "versions",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: APIVersionsAc) -> Result<(), crate::Error> {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIVersions,
> for APIVersionsAc {
    fn from_optionable(
        value: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIVersions,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIVersions,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIVersions,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi::Resource for APIVersionsAc {
    const API_VERSION: &'static str = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIVersions as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIVersions as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIVersions as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIVersions as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIVersions as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIVersions as k8s_openapi::Resource>::Scope;
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_apiversionsac() {
    crate::testutil::roundtrip_test::<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIVersions,
    >();
}
