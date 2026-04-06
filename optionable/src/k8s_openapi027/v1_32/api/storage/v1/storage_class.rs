#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// StorageClass describes the parameters for a class of storage for which PersistentVolumes can be dynamically provisioned.
///
/// StorageClasses are non-namespaced; the name of the storage class according to etcd is in ObjectMeta.Name.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StorageClassAc {
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
    /// allowVolumeExpansion shows whether the storage class allow volume expand.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_volume_expansion: Option<bool>,
    /// allowedTopologies restrict the node topologies where volumes can be dynamically provisioned. Each volume plugin defines its own supported topology specifications. An empty TopologySelectorTerm list means there is no topology restriction. This field is only honored by servers that enable the VolumeScheduling feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_topologies: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::TopologySelectorTerm as crate::Optionable>::Optioned,
        >,
    >,
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// mountOptions controls the mountOptions for dynamically provisioned PersistentVolumes of this storage class. e.g. \["ro", "soft"\]. Not validated - mount of the PVs will simply fail if one is invalid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<std::vec::Vec<std::string::String>>,
    /// parameters holds the parameters for the provisioner that should create volumes of this storage class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
    /// provisioner indicates the type of the provisioner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioner: Option<std::string::String>,
    /// reclaimPolicy controls the reclaimPolicy for dynamically provisioned PersistentVolumes of this storage class. Defaults to Delete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reclaim_policy: Option<std::string::String>,
    /// volumeBindingMode indicates how PersistentVolumeClaims should be provisioned and bound.  When unset, VolumeBindingImmediate is used. This field is only honored by servers that enable the VolumeScheduling feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_binding_mode: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::storage::v1::StorageClass {
    type Optioned = StorageClassAc;
}
#[automatically_derived]
impl crate::Optionable for StorageClassAc {
    type Optioned = StorageClassAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::storage::v1::StorageClass {
    fn into_optioned(self) -> StorageClassAc {
        StorageClassAc {
            api_version: Default::default(),
            kind: Default::default(),
            allow_volume_expansion: self.allow_volume_expansion,
            allowed_topologies: crate::OptionableConvert::into_optioned(
                self.allowed_topologies,
            ),
            metadata: self.metadata,
            mount_options: self.mount_options,
            parameters: self.parameters,
            provisioner: Some(self.provisioner),
            reclaim_policy: self.reclaim_policy,
            volume_binding_mode: self.volume_binding_mode,
        }
    }
    fn try_from_optioned(value: StorageClassAc) -> Result<Self, crate::Error> {
        Ok(Self {
            allow_volume_expansion: value.allow_volume_expansion,
            allowed_topologies: crate::OptionableConvert::try_from_optioned(
                value.allowed_topologies,
            )?,
            metadata: value.metadata,
            mount_options: value.mount_options,
            parameters: value.parameters,
            provisioner: value
                .provisioner
                .ok_or(crate::Error {
                    missing_field: "provisioner",
                })?,
            reclaim_policy: value.reclaim_policy,
            volume_binding_mode: value.volume_binding_mode,
        })
    }
    fn merge(&mut self, other: StorageClassAc) -> Result<(), crate::Error> {
        self.allow_volume_expansion = other.allow_volume_expansion;
        crate::OptionableConvert::merge(
            &mut self.allowed_topologies,
            other.allowed_topologies,
        )?;
        self.metadata = other.metadata;
        self.mount_options = other.mount_options;
        self.parameters = other.parameters;
        if let Some(other_value) = other.provisioner {
            self.provisioner = other_value;
        }
        self.reclaim_policy = other.reclaim_policy;
        self.volume_binding_mode = other.volume_binding_mode;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::storage::v1::StorageClass>
for StorageClassAc {
    fn from_optionable(value: k8s_openapi027::api::storage::v1::StorageClass) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::storage::v1::StorageClass, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::storage::v1::StorageClass,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for StorageClassAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::storage::v1::StorageClass as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::storage::v1::StorageClass as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::storage::v1::StorageClass as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::storage::v1::StorageClass as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::storage::v1::StorageClass as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::storage::v1::StorageClass as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for StorageClassAc {
    type Ty = <k8s_openapi027::api::storage::v1::StorageClass as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_storageclassac() {
    crate::testutil::roundtrip_test::<k8s_openapi027::api::storage::v1::StorageClass>();
}
