#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CSIStorageCapacity stores the result of one CSI GetCapacity call. For a given StorageClass, this describes the available capacity in a particular topology segment.  This can be used when considering where to instantiate new PersistentVolumes.
///
/// For example this can express things like: - StorageClass "standard" has "1234 GiB" available in "topology.kubernetes.io/zone=us-east1" - StorageClass "localssd" has "10 GiB" available in "kubernetes.io/hostname=knode-abc123"
///
/// The following three cases all imply that no capacity is available for a certain combination: - no object exists with suitable topology and storage class name - such an object exists, but the capacity is unset - such an object exists, but the capacity is zero
///
/// The producer of these objects can decide which approach is more suitable.
///
/// They are consumed by the kube-scheduler when a CSI driver opts into capacity-aware scheduling with CSIDriverSpec.StorageCapacity. The scheduler compares the MaximumVolumeSize against the requested size of pending volumes to filter out unsuitable nodes. If MaximumVolumeSize is unset, it falls back to a comparison against the less precise Capacity. If that is also unset, the scheduler assumes that capacity is insufficient and tries some other node.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CSIStorageCapacityAc {
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
    /// capacity is the value reported by the CSI driver in its GetCapacityResponse for a GetCapacityRequest with topology and parameters that match the previous fields.
    ///
    /// The semantic is currently (CSI spec 1.2) defined as: The available capacity, in bytes, of the storage that can be used to provision volumes. If not set, that information is currently unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<
        <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
    /// maximumVolumeSize is the value reported by the CSI driver in its GetCapacityResponse for a GetCapacityRequest with topology and parameters that match the previous fields.
    ///
    /// This is defined since CSI spec 1.4.0 as the largest size that may be used in a CreateVolumeRequest.capacity_range.required_bytes field to create a volume with the same parameters as those in GetCapacityRequest. The corresponding value in the Kubernetes API is ResourceRequirements.Requests in a volume claim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_volume_size: Option<
        <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
    /// Standard object's metadata. The name has no particular meaning. It must be a DNS subdomain (dots allowed, 253 characters). To ensure that there are no conflicts with other CSI drivers on the cluster, the recommendation is to use csisc-\<uuid\>, a generated name, or a reverse-domain name which ends with the unique CSI driver name.
    ///
    /// Objects are namespaced.
    ///
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// nodeTopology defines which nodes have access to the storage for which capacity was reported. If not set, the storage is not accessible from any node in the cluster. If empty, the storage is accessible from all nodes. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_topology: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    /// storageClassName represents the name of the StorageClass that the reported capacity applies to. It must meet the same requirements as the name of a StorageClass object (non-empty, DNS subdomain). If that object no longer exists, the CSIStorageCapacity object is obsolete and should be removed by its creator. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::storage::v1::CSIStorageCapacity {
    type Optioned = CSIStorageCapacityAc;
}
#[automatically_derived]
impl crate::Optionable for CSIStorageCapacityAc {
    type Optioned = CSIStorageCapacityAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::storage::v1::CSIStorageCapacity {
    fn into_optioned(self) -> CSIStorageCapacityAc {
        CSIStorageCapacityAc {
            api_version: Default::default(),
            kind: Default::default(),
            capacity: crate::OptionableConvert::into_optioned(self.capacity),
            maximum_volume_size: crate::OptionableConvert::into_optioned(
                self.maximum_volume_size,
            ),
            metadata: self.metadata,
            node_topology: crate::OptionableConvert::into_optioned(self.node_topology),
            storage_class_name: Some(self.storage_class_name),
        }
    }
    fn try_from_optioned(value: CSIStorageCapacityAc) -> Result<Self, crate::Error> {
        Ok(Self {
            capacity: crate::OptionableConvert::try_from_optioned(value.capacity)?,
            maximum_volume_size: crate::OptionableConvert::try_from_optioned(
                value.maximum_volume_size,
            )?,
            metadata: value.metadata,
            node_topology: crate::OptionableConvert::try_from_optioned(
                value.node_topology,
            )?,
            storage_class_name: value
                .storage_class_name
                .ok_or(crate::Error {
                    missing_field: "storage_class_name",
                })?,
        })
    }
    fn merge(&mut self, other: CSIStorageCapacityAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.capacity, other.capacity)?;
        crate::OptionableConvert::merge(
            &mut self.maximum_volume_size,
            other.maximum_volume_size,
        )?;
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.node_topology, other.node_topology)?;
        if let Some(other_value) = other.storage_class_name {
            self.storage_class_name = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::storage::v1::CSIStorageCapacity>
for CSIStorageCapacityAc {
    fn from_optionable(
        value: k8s_openapi027::api::storage::v1::CSIStorageCapacity,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::storage::v1::CSIStorageCapacity, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::storage::v1::CSIStorageCapacity,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for CSIStorageCapacityAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::storage::v1::CSIStorageCapacity as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::storage::v1::CSIStorageCapacity as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::storage::v1::CSIStorageCapacity as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::storage::v1::CSIStorageCapacity as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::storage::v1::CSIStorageCapacity as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::storage::v1::CSIStorageCapacity as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for CSIStorageCapacityAc {
    type Ty = <k8s_openapi027::api::storage::v1::CSIStorageCapacity as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_csistoragecapacityac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi027::api::storage::v1::CSIStorageCapacity,
    >();
}
