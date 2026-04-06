#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Endpoints is a collection of endpoints that implement the actual service. Example:
///
///   Name: "mysvc",
///      Subsets: \[
///        {
///          Addresses: \[{"ip": "10.10.1.1"}, {"ip": "10.10.2.2"}\],
///          Ports: \[{"name": "a", "port": 8675}, {"name": "b", "port": 309}\]
///        },
///        {
///          Addresses: \[{"ip": "10.10.3.3"}\],
///          Ports: \[{"name": "a", "port": 93}, {"name": "b", "port": 76}\]
///        },
///     \]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EndpointsAc {
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
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// The set of all endpoints is the union of all subsets. Addresses are placed into subsets according to the IPs they share. A single address with multiple ports, some of which are ready and some of which are not (because they come from different containers) will result in the address being displayed in different subsets for the different ports. No address will appear in both Addresses and NotReadyAddresses in the same subset. Sets of addresses and ports that comprise a service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsets: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::EndpointSubset as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::Endpoints {
    type Optioned = EndpointsAc;
}
#[automatically_derived]
impl crate::Optionable for EndpointsAc {
    type Optioned = EndpointsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::Endpoints {
    fn into_optioned(self) -> EndpointsAc {
        EndpointsAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            subsets: crate::OptionableConvert::into_optioned(self.subsets),
        }
    }
    fn try_from_optioned(value: EndpointsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            subsets: crate::OptionableConvert::try_from_optioned(value.subsets)?,
        })
    }
    fn merge(&mut self, other: EndpointsAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.subsets, other.subsets)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::Endpoints> for EndpointsAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::Endpoints) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::Endpoints, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::Endpoints,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for EndpointsAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::core::v1::Endpoints as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::core::v1::Endpoints as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::core::v1::Endpoints as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::core::v1::Endpoints as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::core::v1::Endpoints as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::core::v1::Endpoints as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for EndpointsAc {
    type Ty = <k8s_openapi027::api::core::v1::Endpoints as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_endpointsac() {
    crate::testutil::roundtrip_test::<k8s_openapi027::api::core::v1::Endpoints>();
}
