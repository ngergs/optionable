#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CSINodeDriverAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocatable: Option<
        <::k8s_openapi027::api::storage::v1::VolumeNodeResources as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    #[serde(rename = "nodeID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topology_keys: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::storage::v1::CSINodeDriver {
    type Optioned = CSINodeDriverAc;
}
#[automatically_derived]
impl crate::Optionable for CSINodeDriverAc {
    type Optioned = CSINodeDriverAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::storage::v1::CSINodeDriver {
    fn into_optioned(self) -> CSINodeDriverAc {
        CSINodeDriverAc {
            allocatable: crate::OptionableConvert::into_optioned(self.allocatable),
            name: Some(self.name),
            node_id: Some(self.node_id),
            topology_keys: self.topology_keys,
        }
    }
    fn try_from_optioned(value: CSINodeDriverAc) -> Result<Self, crate::Error> {
        Ok(Self {
            allocatable: crate::OptionableConvert::try_from_optioned(value.allocatable)?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            node_id: value
                .node_id
                .ok_or(crate::Error {
                    missing_field: "node_id",
                })?,
            topology_keys: value.topology_keys,
        })
    }
    fn merge(&mut self, other: CSINodeDriverAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.allocatable, other.allocatable)?;
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        if let Some(other_value) = other.node_id {
            self.node_id = other_value;
        }
        self.topology_keys = other.topology_keys;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::storage::v1::CSINodeDriver>
for CSINodeDriverAc {
    fn from_optionable(value: k8s_openapi027::api::storage::v1::CSINodeDriver) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::storage::v1::CSINodeDriver, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::storage::v1::CSINodeDriver,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
