#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// APIService represents a server for a particular GroupVersion. Name must be "version.group".
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct APIServiceAc {
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
    /// Spec contains information for locating and communicating with a server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceSpec as crate::Optionable>::Optioned,
    >,
    /// Status contains derived information about an API server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<
        <::k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceStatus as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIService {
    type Optioned = APIServiceAc;
}
#[automatically_derived]
impl crate::Optionable for APIServiceAc {
    type Optioned = APIServiceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIService {
    fn into_optioned(self) -> APIServiceAc {
        APIServiceAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(value: APIServiceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(&mut self, other: APIServiceAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIService,
> for APIServiceAc {
    fn from_optionable(
        value: k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIService,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIService,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIService,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for APIServiceAc {
    const API_VERSION: &'static str = <k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIService as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIService as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIService as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIService as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIService as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIService as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for APIServiceAc {
    type Ty = <k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIService as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_apiserviceac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIService,
    >();
}
