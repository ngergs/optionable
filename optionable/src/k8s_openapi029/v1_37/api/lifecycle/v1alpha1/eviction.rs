#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Eviction initiates an eviction process, which should ideally result in a graceful eviction of a .spec.target (e.g. termination of a pod).
///
/// The evictionrequest-controller observes intents of all EvictionRequests and transforms them into Evictions. It manages the Eviction lifecycle. Requesters are preserved in .status.requesters even after they have withdrawn their request. If all requesters withdraw their eviction intent for a common target, the eviction will be canceled. Once all EvictionRequest corresponding to this Eviction .spec.target have been removed, this Eviction object will eventually be garbage collected.
///
/// If the target is a pod, the .status.targetResponders is populated from Pod's .spec.evictionResponders.
///
/// Responders should observe and communicate through the .status to help with the eviction of the target when they see their state == Active in .status.targetResponders. ResponderStatus struct should then be periodically updated to indicate the progress or completion of the eviction process by each responder in .status.responders. If .status.responders\[\].heartbeatTime is not updated within the heartbeat deadline defined by the Eviction API (currently 20 minutes), the eviction is passed over to the next responder with a lower priority.
///
/// If there are no other responders and the target is a pod, the last default imperative-eviction.k8s.io/evictor responder with a priority of 100 will evict the pod using the imperative Eviction API (pods/\<name\>/eviction subresource).
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvictionAc {
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
    /// metadata is the standard object metadata; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata. .metadata.name set by the evictionrequest-controller is purely informative and subject to change. .spec.target field should be used to identify the target precisesly.
    ///
    /// The requester and responder names will be used as label keys and added to the labels of the eviction in one of the following formats: 1. acme.io/foo: "requester" 2. acme.io/foo: "responder" 3. acme.io/foo: "requester-responder"
    ///
    /// Please see EvictionParticipantRole for available role label values.
    pub metadata: ::k8s_openapi029::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// spec defines the eviction specification. https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi029::api::lifecycle::v1alpha1::EvictionSpec as crate::Optionable>::Optioned,
    >,
    /// status represents the most recently observed status of the eviction. Populated by responders and evictionrequest-controller. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<
        <::k8s_openapi029::api::lifecycle::v1alpha1::EvictionStatus as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::lifecycle::v1alpha1::Eviction {
    type Optioned = EvictionAc;
}
#[automatically_derived]
impl crate::Optionable for EvictionAc {
    type Optioned = EvictionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi029::api::lifecycle::v1alpha1::Eviction {
    fn into_optioned(self) -> EvictionAc {
        EvictionAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(value: EvictionAc) -> Result<Self, crate::Error> {
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
    fn merge(&mut self, other: EvictionAc) -> Result<(), crate::Error> {
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
impl crate::OptionedConvert<k8s_openapi029::api::lifecycle::v1alpha1::Eviction>
for EvictionAc {
    fn from_optionable(
        value: k8s_openapi029::api::lifecycle::v1alpha1::Eviction,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi029::api::lifecycle::v1alpha1::Eviction, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::lifecycle::v1alpha1::Eviction,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::Resource for EvictionAc {
    const API_VERSION: &'static str = <k8s_openapi029::api::lifecycle::v1alpha1::Eviction as k8s_openapi029::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi029::api::lifecycle::v1alpha1::Eviction as k8s_openapi029::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi029::api::lifecycle::v1alpha1::Eviction as k8s_openapi029::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi029::api::lifecycle::v1alpha1::Eviction as k8s_openapi029::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi029::api::lifecycle::v1alpha1::Eviction as k8s_openapi029::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi029::api::lifecycle::v1alpha1::Eviction as k8s_openapi029::Resource>::Scope;
}
impl k8s_openapi029::Metadata for EvictionAc {
    type Ty = <k8s_openapi029::api::lifecycle::v1alpha1::Eviction as k8s_openapi029::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi029::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi029::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_evictionac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi029::api::lifecycle::v1alpha1::Eviction,
    >();
}
impl k8s_openapi029::DeepMerge for EvictionAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        k8s_openapi029::DeepMerge::merge_from(&mut self.spec, other.spec);
        k8s_openapi029::DeepMerge::merge_from(&mut self.status, other.status);
    }
}
