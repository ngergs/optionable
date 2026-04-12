#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// APIGroup contains the name, the supported versions, and the preferred version of a group.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct APIGroupAc {
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
    /// name is the name of the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// preferredVersion is the version preferred by the API server, which probably is the storage version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_version: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery as crate::Optionable>::Optioned,
    >,
    /// a map of client CIDR to server address that is serving this group. This is to help clients reach servers in the most network-efficient way possible. Clients can use the appropriate server address as per the CIDR that they match. In case of multiple matches, clients should use the longest matching CIDR. The server returns only those CIDRs that it thinks that the client can match. For example: the master will return an internal IP CIDR only, if the client reaches the server using an internal IP. Server looks at X-Forwarded-For header or X-Real-Ip header or request.RemoteAddr (in that order) to get the client IP.
    #[serde(rename = "serverAddressByClientCIDRs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_address_by_client_cidrs: Option<
        std::vec::Vec<
            <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ServerAddressByClientCIDR as crate::Optionable>::Optioned,
        >,
    >,
    /// versions are the versions supported in this group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<
        std::vec::Vec<
            <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::GroupVersionForDiscovery as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroup {
    type Optioned = APIGroupAc;
}
#[automatically_derived]
impl crate::Optionable for APIGroupAc {
    type Optioned = APIGroupAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroup {
    fn into_optioned(self) -> APIGroupAc {
        APIGroupAc {
            api_version: Default::default(),
            kind: Default::default(),
            name: Some(self.name),
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
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
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
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.preferred_version.is_none() {
            self.preferred_version = crate::OptionableConvert::try_from_optioned(
                other.preferred_version,
            )?;
        } else if let Some(self_value) = self.preferred_version.as_mut()
            && let Some(other_value) = other.preferred_version
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.server_address_by_client_cidrs.is_none() {
            self.server_address_by_client_cidrs = crate::OptionableConvert::try_from_optioned(
                other.server_address_by_client_cidrs,
            )?;
        } else if let Some(self_value) = self.server_address_by_client_cidrs.as_mut()
            && let Some(other_value) = other.server_address_by_client_cidrs
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.versions {
            self.versions = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroup>
for APIGroupAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroup,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroup,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroup,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for APIGroupAc {
    const API_VERSION: &'static str = <k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroup as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroup as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroup as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroup as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroup as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroup as k8s_openapi027::Resource>::Scope;
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_apigroupac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIGroup,
    >();
}
