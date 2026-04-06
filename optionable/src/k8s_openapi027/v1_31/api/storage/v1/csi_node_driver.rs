#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CSINodeDriver holds information about the specification of one CSI driver installed on a node
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CSINodeDriverAc {
    /// allocatable represents the volume resources of a node that are available for scheduling. This field is beta.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocatable: Option<
        <::k8s_openapi027::api::storage::v1::VolumeNodeResources as crate::Optionable>::Optioned,
    >,
    /// name represents the name of the CSI driver that this object refers to. This MUST be the same name returned by the CSI GetPluginName() call for that driver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// nodeID of the node from the driver point of view. This field enables Kubernetes to communicate with storage systems that do not share the same nomenclature for nodes. For example, Kubernetes may refer to a given node as "node1", but the storage system may refer to the same node as "nodeA". When Kubernetes issues a command to the storage system to attach a volume to a specific node, it can use this field to refer to the node name using the ID that the storage system will understand, e.g. "nodeA" instead of "node1". This field is required.
    #[serde(rename = "nodeID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<std::string::String>,
    /// topologyKeys is the list of keys supported by the driver. When a driver is initialized on a cluster, it provides a set of topology keys that it understands (e.g. "company.com/zone", "company.com/region"). When a driver is initialized on a node, it provides the same topology keys along with values. Kubelet will expose these topology keys as labels on its own node object. When Kubernetes does topology aware provisioning, it can use this list to determine which labels it should retrieve from the node object and pass back to the driver. It is possible for different nodes to use different topology keys. This can be empty if driver does not support topology.
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
