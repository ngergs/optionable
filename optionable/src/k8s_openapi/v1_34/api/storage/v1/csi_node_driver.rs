pub struct CSINodeDriverOpt {
    pub allocatable: <Option<
        ::k8s_openapi::api::storage::v1::VolumeNodeResources,
    > as crate::Optionable>::Optioned,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub node_id: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub topology_keys: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::CSINodeDriver {
    type Optioned = CSINodeDriverOpt;
}
#[automatically_derived]
impl crate::Optionable for CSINodeDriverOpt {
    type Optioned = CSINodeDriverOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::storage::v1::CSINodeDriver {
    fn into_optioned(self) -> CSINodeDriverOpt {
        CSINodeDriverOpt {
            allocatable: <Option<
                ::k8s_openapi::api::storage::v1::VolumeNodeResources,
            > as crate::OptionableConvert>::into_optioned(self.allocatable),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            node_id: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.node_id,
                ),
            ),
            topology_keys: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.topology_keys),
        }
    }
    fn try_from_optioned(
        value: CSINodeDriverOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            allocatable: <Option<
                ::k8s_openapi::api::storage::v1::VolumeNodeResources,
            > as crate::OptionableConvert>::try_from_optioned(value.allocatable)?,
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            node_id: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .node_id
                    .ok_or(crate::optionable::Error {
                        missing_field: "node_id",
                    })?,
            )?,
            topology_keys: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.topology_keys)?,
        })
    }
    fn merge(
        &mut self,
        other: CSINodeDriverOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::storage::v1::VolumeNodeResources,
        > as crate::OptionableConvert>::merge(&mut self.allocatable, other.allocatable)?;
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        if let Some(other_value) = other.node_id {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.node_id,
                other_value,
            )?;
        }
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.topology_keys,
            other.topology_keys,
        )?;
        Ok(())
    }
}
