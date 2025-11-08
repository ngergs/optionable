#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ClusterTrustBundleAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi::api::certificates::v1beta1::ClusterTrustBundleSpec as crate::Optionable>::Optioned,
    >,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    pub phantom: std::marker::PhantomData<ClusterTrustBundleAc>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::certificates::v1beta1::ClusterTrustBundle {
    type Optioned = ClusterTrustBundleAc;
}
#[automatically_derived]
impl crate::Optionable for ClusterTrustBundleAc {
    type Optioned = ClusterTrustBundleAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::certificates::v1beta1::ClusterTrustBundle {
    fn into_optioned(self) -> ClusterTrustBundleAc {
        ClusterTrustBundleAc {
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(value: ClusterTrustBundleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::Error {
                        missing_field: "spec",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: ClusterTrustBundleAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        Ok(())
    }
}
impl k8s_openapi::Resource for ClusterTrustBundleAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::certificates::v1beta1::ClusterTrustBundle as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::certificates::v1beta1::ClusterTrustBundle as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::certificates::v1beta1::ClusterTrustBundle as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::certificates::v1beta1::ClusterTrustBundle as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::certificates::v1beta1::ClusterTrustBundle as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::certificates::v1beta1::ClusterTrustBundle as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for ClusterTrustBundleAc {
    type Ty = <::k8s_openapi::api::certificates::v1beta1::ClusterTrustBundle as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
