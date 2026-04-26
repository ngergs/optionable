#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ControllerRevision implements an immutable snapshot of state data. Clients are responsible for serializing and deserializing the objects that contain their internal state. Once a ControllerRevision has been successfully created, it can not be updated. The API Server will fail validation of all requests that attempt to mutate the Data field. ControllerRevisions may, however, be deleted. Note that, due to its use by both the DaemonSet and StatefulSet controllers for update and rollback, this object is beta. However, it may be subject to name and representation changes in future releases, and clients should not depend on its stability. It is primarily for internal use by controllers.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ControllerRevisionAc {
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
    /// Data is the serialized representation of the state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<
        <::k8s_openapi027::apimachinery::pkg::runtime::RawExtension as crate::Optionable>::Optioned,
    >,
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// Revision indicates the revision of the state represented by Data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::apps::v1::ControllerRevision {
    type Optioned = ControllerRevisionAc;
}
#[automatically_derived]
impl crate::Optionable for ControllerRevisionAc {
    type Optioned = ControllerRevisionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::apps::v1::ControllerRevision {
    fn into_optioned(self) -> ControllerRevisionAc {
        ControllerRevisionAc {
            api_version: Default::default(),
            kind: Default::default(),
            data: crate::OptionableConvert::into_optioned(self.data),
            metadata: self.metadata,
            revision: Some(self.revision),
        }
    }
    fn try_from_optioned(value: ControllerRevisionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            data: crate::OptionableConvert::try_from_optioned(value.data)?,
            metadata: value.metadata,
            revision: value
                .revision
                .ok_or(crate::Error {
                    missing_field: "revision",
                })?,
        })
    }
    fn merge(&mut self, other: ControllerRevisionAc) -> Result<(), crate::Error> {
        if self.data.is_none() {
            self.data = crate::OptionableConvert::try_from_optioned(other.data)?;
        } else if let Some(self_value) = self.data.as_mut()
            && let Some(other_value) = other.data
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        self.metadata = other.metadata;
        if let Some(other_value) = other.revision {
            self.revision = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::apps::v1::ControllerRevision>
for ControllerRevisionAc {
    fn from_optionable(
        value: k8s_openapi027::api::apps::v1::ControllerRevision,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::apps::v1::ControllerRevision, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apps::v1::ControllerRevision,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for ControllerRevisionAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::apps::v1::ControllerRevision as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::apps::v1::ControllerRevision as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::apps::v1::ControllerRevision as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::apps::v1::ControllerRevision as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::apps::v1::ControllerRevision as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::apps::v1::ControllerRevision as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for ControllerRevisionAc {
    type Ty = <k8s_openapi027::api::apps::v1::ControllerRevision as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_controllerrevisionac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi027::api::apps::v1::ControllerRevision,
    >();
}
impl k8s_openapi027::DeepMerge for ControllerRevisionAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.data, other.data);
        k8s_openapi027::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        k8s_openapi027::DeepMerge::merge_from(&mut self.revision, other.revision);
    }
}
