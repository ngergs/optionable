#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct APIGroupAc {
    #[serde(
        serialize_with = "crate::k8s_openapi026::serialize_api_version",
        deserialize_with = "crate::k8s_openapi026::deserialize_api_version"
    )]
    pub api_version: std::marker::PhantomData<Self>,
    #[serde(
        serialize_with = "crate::k8s_openapi026::serialize_kind",
        deserialize_with = "crate::k8s_openapi026::deserialize_kind"
    )]
    pub kind: std::marker::PhantomData<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_version: <Option<
        ::k8s_openapi026::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery,
    > as crate::Optionable>::Optioned,
    #[serde(rename = "serverAddressByClientCIDRs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_address_by_client_cidrs: <Option<
        std::vec::Vec<
            ::k8s_openapi026::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<
        <std::vec::Vec<
            ::k8s_openapi026::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::apimachinery::pkg::apis::meta::v1::APIGroup {
    type Optioned = APIGroupAc;
}
#[automatically_derived]
impl crate::Optionable for APIGroupAc {
    type Optioned = APIGroupAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::apimachinery::pkg::apis::meta::v1::APIGroup {
    fn into_optioned(self) -> APIGroupAc {
        APIGroupAc {
            api_version: Default::default(),
            kind: Default::default(),
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
    fn try_from_optioned(value: APIGroupAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
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
                    .ok_or(crate::Error {
                        missing_field: "versions",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: APIGroupAc) -> Result<(), crate::Error> {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::apimachinery::pkg::apis::meta::v1::APIGroup>
for APIGroupAc {
    fn from_optionable(
        value: k8s_openapi026::apimachinery::pkg::apis::meta::v1::APIGroup,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::apimachinery::pkg::apis::meta::v1::APIGroup,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::apimachinery::pkg::apis::meta::v1::APIGroup,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi026::Resource for APIGroupAc {
    const API_VERSION: &'static str = <k8s_openapi026::apimachinery::pkg::apis::meta::v1::APIGroup as k8s_openapi026::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi026::apimachinery::pkg::apis::meta::v1::APIGroup as k8s_openapi026::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi026::apimachinery::pkg::apis::meta::v1::APIGroup as k8s_openapi026::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi026::apimachinery::pkg::apis::meta::v1::APIGroup as k8s_openapi026::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi026::apimachinery::pkg::apis::meta::v1::APIGroup as k8s_openapi026::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi026::apimachinery::pkg::apis::meta::v1::APIGroup as k8s_openapi026::Resource>::Scope;
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_apigroupac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi026::apimachinery::pkg::apis::meta::v1::APIGroup,
    >();
}
