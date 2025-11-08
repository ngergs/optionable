#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct CSIStorageCapacityAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: <Option<
        ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_volume_size: <Option<
        ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_topology: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    pub phantom: std::marker::PhantomData<CSIStorageCapacityAc>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::CSIStorageCapacity {
    type Optioned = CSIStorageCapacityAc;
}
#[automatically_derived]
impl crate::Optionable for CSIStorageCapacityAc {
    type Optioned = CSIStorageCapacityAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::storage::v1::CSIStorageCapacity {
    fn into_optioned(self) -> CSIStorageCapacityAc {
        CSIStorageCapacityAc {
            capacity: crate::OptionableConvert::into_optioned(self.capacity),
            maximum_volume_size: crate::OptionableConvert::into_optioned(
                self.maximum_volume_size,
            ),
            metadata: self.metadata,
            node_topology: crate::OptionableConvert::into_optioned(self.node_topology),
            storage_class_name: Some(
                crate::OptionableConvert::into_optioned(self.storage_class_name),
            ),
            phantom: Default::default(),
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
            storage_class_name: crate::OptionableConvert::try_from_optioned(
                value
                    .storage_class_name
                    .ok_or(crate::Error {
                        missing_field: "storage_class_name",
                    })?,
            )?,
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
            crate::OptionableConvert::merge(&mut self.storage_class_name, other_value)?;
        }
        Ok(())
    }
}
impl k8s_openapi::Resource for CSIStorageCapacityAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::storage::v1::CSIStorageCapacity as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::storage::v1::CSIStorageCapacity as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::storage::v1::CSIStorageCapacity as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::storage::v1::CSIStorageCapacity as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::storage::v1::CSIStorageCapacity as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::storage::v1::CSIStorageCapacity as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for CSIStorageCapacityAc {
    type Ty = <::k8s_openapi::api::storage::v1::CSIStorageCapacity as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
