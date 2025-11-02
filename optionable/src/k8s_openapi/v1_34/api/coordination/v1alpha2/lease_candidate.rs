#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaseCandidateAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: <Option<
        ::k8s_openapi::api::coordination::v1alpha2::LeaseCandidateSpec,
    > as crate::Optionable>::Optioned,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    phantom: std::marker::PhantomData<LeaseCandidateAc>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::coordination::v1alpha2::LeaseCandidate {
    type Optioned = LeaseCandidateAc;
}
#[automatically_derived]
impl crate::Optionable for LeaseCandidateAc {
    type Optioned = LeaseCandidateAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::coordination::v1alpha2::LeaseCandidate {
    fn into_optioned(self) -> LeaseCandidateAc {
        LeaseCandidateAc {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(
        value: LeaseCandidateAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(
        &mut self,
        other: LeaseCandidateAc,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
impl k8s_openapi::Resource for LeaseCandidateAc {
    const API_VERSION: &'static str = "coordination.k8s.io/v1alpha2";
    const GROUP: &'static str = "coordination.k8s.io";
    const KIND: &'static str = "LeaseCandidate";
    const VERSION: &'static str = "v1alpha2";
    const URL_PATH_SEGMENT: &'static str = "leasecandidates";
    type Scope = k8s_openapi::NamespaceResourceScope;
}
impl k8s_openapi::Metadata for LeaseCandidateAc {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
