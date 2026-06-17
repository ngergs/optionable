#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ResourcePoolStatusRequest triggers a one-time calculation of resource pool status based on the provided filters. Once status is set, the request is considered complete and will not be reprocessed. Users should delete and recreate requests to get updated information.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourcePoolStatusRequestAc {
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
    /// Standard object metadata
    pub metadata: ::k8s_openapi028::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// Spec defines the filters for which pools to include in the status. The spec is immutable once created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequestSpec as crate::Optionable>::Optioned,
    >,
    /// Status is populated by the controller with the calculated pool status. When status is non-nil, the request is considered complete and the entire object becomes immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<
        <::k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequestStatus as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequest {
    type Optioned = ResourcePoolStatusRequestAc;
}
#[automatically_derived]
impl crate::Optionable for ResourcePoolStatusRequestAc {
    type Optioned = ResourcePoolStatusRequestAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequest {
    fn into_optioned(self) -> ResourcePoolStatusRequestAc {
        ResourcePoolStatusRequestAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(
        value: ResourcePoolStatusRequestAc,
    ) -> Result<Self, crate::Error> {
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
    fn merge(&mut self, other: ResourcePoolStatusRequestAc) -> Result<(), crate::Error> {
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
    k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequest,
> for ResourcePoolStatusRequestAc {
    fn from_optionable(
        value: k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequest,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequest,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequest,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::Resource for ResourcePoolStatusRequestAc {
    const API_VERSION: &'static str = <k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequest as k8s_openapi028::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequest as k8s_openapi028::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequest as k8s_openapi028::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequest as k8s_openapi028::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequest as k8s_openapi028::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequest as k8s_openapi028::Resource>::Scope;
}
impl k8s_openapi028::Metadata for ResourcePoolStatusRequestAc {
    type Ty = <k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequest as k8s_openapi028::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi028::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi028::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_resourcepoolstatusrequestac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequest,
    >();
}
impl k8s_openapi028::DeepMerge for ResourcePoolStatusRequestAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi028::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        k8s_openapi028::DeepMerge::merge_from(&mut self.spec, other.spec);
        k8s_openapi028::DeepMerge::merge_from(&mut self.status, other.status);
    }
}
