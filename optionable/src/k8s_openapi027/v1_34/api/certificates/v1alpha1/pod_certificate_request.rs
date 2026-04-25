#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodCertificateRequest encodes a pod requesting a certificate from a given signer.
///
/// Kubelets use this API to implement podCertificate projected volumes
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodCertificateRequestAc {
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
    /// metadata contains the object metadata.
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// spec contains the details about the certificate being requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequestSpec as crate::Optionable>::Optioned,
    >,
    /// status contains the issued certificate, and a standard set of conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<
        <::k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequestStatus as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequest {
    type Optioned = PodCertificateRequestAc;
}
#[automatically_derived]
impl crate::Optionable for PodCertificateRequestAc {
    type Optioned = PodCertificateRequestAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequest {
    fn into_optioned(self) -> PodCertificateRequestAc {
        PodCertificateRequestAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(value: PodCertificateRequestAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::Error {
                        missing_field: "spec",
                    })?,
            )?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(&mut self, other: PodCertificateRequestAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        if self.status.is_none() {
            self.status = crate::OptionableConvert::try_from_optioned(other.status)?;
        } else if let Some(self_value) = self.status.as_mut()
            && let Some(other_value) = other.status
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequest,
> for PodCertificateRequestAc {
    fn from_optionable(
        value: k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequest,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequest,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequest,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for PodCertificateRequestAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequest as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequest as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequest as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequest as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequest as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequest as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for PodCertificateRequestAc {
    type Ty = <k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequest as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_podcertificaterequestac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequest,
    >();
}
impl k8s_openapi027::DeepMerge for PodCertificateRequestAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.api_version, other.api_version);
        k8s_openapi027::DeepMerge::merge_from(&mut self.kind, other.kind);
        k8s_openapi027::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        k8s_openapi027::DeepMerge::merge_from(&mut self.spec, other.spec);
        k8s_openapi027::DeepMerge::merge_from(&mut self.status, other.status);
    }
}
