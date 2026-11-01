#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// EvictionRequest defines a request that should ideally result in a graceful eviction of a .spec.target (e.g. termination of a pod).
///
/// The evictionrequest-controller observes intents of all EvictionRequests and transforms them into Evictions.
///   - .spec.requester is set as a label on the Eviction for easier lookup.
///   - Each target can have a set of responders assigned to it. Eviction objects are observed by
///     these responders, who implement the eviction logic and update the Eviction's status with
///     progress.
///
/// There is many-to-many relationship between EvictionRequests and Evictions in general. And many-to-one if the target is a  pod.
///
/// If all requesters withdraw their eviction intent for a common target, the eviction will be canceled. Deleting an EvictionRequest also counts as a withdrawal. Once all EvictionRequest of a target are removed, the corresponding Evictions are eventually garbage collected.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvictionRequestAc {
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
    /// metadata is the standard object metadata; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata.
    pub metadata: ::k8s_openapi029::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// spec defines the eviction request specification. https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequestSpec as crate::Optionable>::Optioned,
    >,
    /// status represents the most recently observed status of the eviction request. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<
        <::k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequestStatus as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequest {
    type Optioned = EvictionRequestAc;
}
#[automatically_derived]
impl crate::Optionable for EvictionRequestAc {
    type Optioned = EvictionRequestAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequest {
    fn into_optioned(self) -> EvictionRequestAc {
        EvictionRequestAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(value: EvictionRequestAc) -> Result<Self, crate::Error> {
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
    fn merge(&mut self, other: EvictionRequestAc) -> Result<(), crate::Error> {
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
impl crate::OptionedConvert<k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequest>
for EvictionRequestAc {
    fn from_optionable(
        value: k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequest,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequest,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequest,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::Resource for EvictionRequestAc {
    const API_VERSION: &'static str = <k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequest as k8s_openapi029::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequest as k8s_openapi029::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequest as k8s_openapi029::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequest as k8s_openapi029::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequest as k8s_openapi029::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequest as k8s_openapi029::Resource>::Scope;
}
impl k8s_openapi029::Metadata for EvictionRequestAc {
    type Ty = <k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequest as k8s_openapi029::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi029::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi029::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_evictionrequestac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequest,
    >();
}
impl k8s_openapi029::DeepMerge for EvictionRequestAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        k8s_openapi029::DeepMerge::merge_from(&mut self.spec, other.spec);
        k8s_openapi029::DeepMerge::merge_from(&mut self.status, other.status);
    }
}
