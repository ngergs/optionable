#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSINodeDriverAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocatable: <Option<
        ::k8s_openapi::api::storage::v1::VolumeNodeResources,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topology_keys: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::CSINodeDriver {
    type Optioned = CSINodeDriverAc;
}
#[automatically_derived]
impl crate::Optionable for CSINodeDriverAc {
    type Optioned = CSINodeDriverAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::storage::v1::CSINodeDriver {
    fn into_optioned(self) -> CSINodeDriverAc {
        CSINodeDriverAc {
            allocatable: crate::OptionableConvert::into_optioned(self.allocatable),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            node_id: Some(crate::OptionableConvert::into_optioned(self.node_id)),
            topology_keys: crate::OptionableConvert::into_optioned(self.topology_keys),
        }
    }
    fn try_from_optioned(
        value: CSINodeDriverAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            allocatable: crate::OptionableConvert::try_from_optioned(value.allocatable)?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            node_id: crate::OptionableConvert::try_from_optioned(
                value
                    .node_id
                    .ok_or(crate::optionable::Error {
                        missing_field: "node_id",
                    })?,
            )?,
            topology_keys: crate::OptionableConvert::try_from_optioned(
                value.topology_keys,
            )?,
        })
    }
    fn merge(&mut self, other: CSINodeDriverAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.allocatable, other.allocatable)?;
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        if let Some(other_value) = other.node_id {
            crate::OptionableConvert::merge(&mut self.node_id, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.topology_keys, other.topology_keys)?;
        Ok(())
    }
}
