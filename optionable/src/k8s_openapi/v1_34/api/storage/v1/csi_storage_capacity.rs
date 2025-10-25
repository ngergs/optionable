pub struct CSIStorageCapacityOpt {
    pub capacity: <Option<
        ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
    pub maximum_volume_size: <Option<
        ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub node_topology: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    pub storage_class_name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::storage::v1::csi_storage_capacity::CSIStorageCapacity {
    type Optioned = CSIStorageCapacityOpt;
}
#[automatically_derived]
impl crate::Optionable for CSIStorageCapacityOpt {
    type Optioned = CSIStorageCapacityOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::storage::v1::csi_storage_capacity::CSIStorageCapacity {
    fn into_optioned(self) -> CSIStorageCapacityOpt {
        CSIStorageCapacityOpt {
            capacity: <Option<
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            > as crate::OptionableConvert>::into_optioned(self.capacity),
            maximum_volume_size: <Option<
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            > as crate::OptionableConvert>::into_optioned(self.maximum_volume_size),
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            node_topology: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
            > as crate::OptionableConvert>::into_optioned(self.node_topology),
            storage_class_name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.storage_class_name,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: CSIStorageCapacityOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            capacity: <Option<
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            > as crate::OptionableConvert>::try_from_optioned(value.capacity)?,
            maximum_volume_size: <Option<
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            > as crate::OptionableConvert>::try_from_optioned(
                value.maximum_volume_size,
            )?,
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            node_topology: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
            > as crate::OptionableConvert>::try_from_optioned(value.node_topology)?,
            storage_class_name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .storage_class_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "storage_class_name",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: CSIStorageCapacityOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        > as crate::OptionableConvert>::merge(&mut self.capacity, other.capacity)?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        > as crate::OptionableConvert>::merge(
            &mut self.maximum_volume_size,
            other.maximum_volume_size,
        )?;
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
        > as crate::OptionableConvert>::merge(
            &mut self.node_topology,
            other.node_topology,
        )?;
        if let Some(other_value) = other.storage_class_name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.storage_class_name,
                other_value,
            )?;
        }
        Ok(())
    }
}
