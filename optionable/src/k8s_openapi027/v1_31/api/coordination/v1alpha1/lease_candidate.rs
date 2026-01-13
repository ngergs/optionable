#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LeaseCandidateAc {
    #[serde(
        serialize_with = "crate::k8s_openapi027::serialize_api_version",
        deserialize_with = "crate::k8s_openapi027::deserialize_api_version"
    )]
    pub api_version: std::marker::PhantomData<Self>,
    #[serde(
        serialize_with = "crate::k8s_openapi027::serialize_kind",
        deserialize_with = "crate::k8s_openapi027::deserialize_kind"
    )]
    pub kind: std::marker::PhantomData<Self>,
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: <Option<
        ::k8s_openapi027::api::coordination::v1alpha1::LeaseCandidateSpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::coordination::v1alpha1::LeaseCandidate {
    type Optioned = LeaseCandidateAc;
}
#[automatically_derived]
impl crate::Optionable for LeaseCandidateAc {
    type Optioned = LeaseCandidateAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::coordination::v1alpha1::LeaseCandidate {
    fn into_optioned(self) -> LeaseCandidateAc {
        LeaseCandidateAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(value: LeaseCandidateAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(&mut self, other: LeaseCandidateAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::coordination::v1alpha1::LeaseCandidate>
for LeaseCandidateAc {
    fn from_optionable(
        value: k8s_openapi027::api::coordination::v1alpha1::LeaseCandidate,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::coordination::v1alpha1::LeaseCandidate,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::coordination::v1alpha1::LeaseCandidate,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for LeaseCandidateAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::coordination::v1alpha1::LeaseCandidate as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::coordination::v1alpha1::LeaseCandidate as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::coordination::v1alpha1::LeaseCandidate as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::coordination::v1alpha1::LeaseCandidate as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::coordination::v1alpha1::LeaseCandidate as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::coordination::v1alpha1::LeaseCandidate as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for LeaseCandidateAc {
    type Ty = <k8s_openapi027::api::coordination::v1alpha1::LeaseCandidate as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_leasecandidateac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi027::api::coordination::v1alpha1::LeaseCandidate,
    >();
}
