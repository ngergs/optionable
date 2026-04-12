#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ResourceSlice represents one or more resources in a pool of similar resources, managed by a common driver. A pool may span more than one ResourceSlice, and exactly how many ResourceSlices comprise a pool is determined by the driver.
///
/// At the moment, the only supported resources are devices with attributes and capacities. Each device in a given pool, regardless of how many ResourceSlices, must have a unique name. The ResourceSlice in which a device gets published may change over time. The unique identifier for a device is the tuple \<driver name\>, \<pool name\>, \<device name\>.
///
/// Whenever a driver needs to update a pool, it increments the pool.Spec.Pool.Generation number and updates all ResourceSlices with that new number and new resource definitions. A consumer must only use ResourceSlices with the highest generation number and ignore all others.
///
/// When allocating all resources in a pool matching certain criteria or when looking for the best solution among several different alternatives, a consumer should check the number of ResourceSlices in a pool (included in each ResourceSlice) to determine whether its view of a pool is complete and if not, should wait until the driver has completed updating the pool.
///
/// For resources that are not local to a node, the node name is not set. Instead, the driver may use a node selector to specify where the devices are available.
///
/// This is an alpha type and requires enabling the DynamicResourceAllocation feature gate.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceSliceAc {
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
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// Contains the information published by the driver.
    ///
    /// Changing the spec automatically increments the metadata.generation number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi027::api::resource::v1alpha3::ResourceSliceSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1alpha3::ResourceSlice {
    type Optioned = ResourceSliceAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceSliceAc {
    type Optioned = ResourceSliceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1alpha3::ResourceSlice {
    fn into_optioned(self) -> ResourceSliceAc {
        ResourceSliceAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
        }
    }
    fn try_from_optioned(value: ResourceSliceAc) -> Result<Self, crate::Error> {
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
    fn merge(&mut self, other: ResourceSliceAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.spec {
            self.spec = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1alpha3::ResourceSlice>
for ResourceSliceAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::ResourceSlice,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1alpha3::ResourceSlice, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::ResourceSlice,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for ResourceSliceAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::resource::v1alpha3::ResourceSlice as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::resource::v1alpha3::ResourceSlice as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::resource::v1alpha3::ResourceSlice as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::resource::v1alpha3::ResourceSlice as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::resource::v1alpha3::ResourceSlice as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::resource::v1alpha3::ResourceSlice as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for ResourceSliceAc {
    type Ty = <k8s_openapi027::api::resource::v1alpha3::ResourceSlice as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_resourcesliceac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi027::api::resource::v1alpha3::ResourceSlice,
    >();
}
