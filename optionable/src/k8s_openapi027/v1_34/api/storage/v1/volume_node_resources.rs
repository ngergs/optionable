#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// VolumeNodeResources is a set of resource limits for scheduling of volumes.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VolumeNodeResourcesAc {
    /// count indicates the maximum number of unique volumes managed by the CSI driver that can be used on a node. A volume that is both attached and mounted on a node is considered to be used once, not twice. The same rule applies for a unique volume that is shared among multiple pods on the same node. If this field is not specified, then the supported number of volumes on this node is unbounded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::storage::v1::VolumeNodeResources {
    type Optioned = VolumeNodeResourcesAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeNodeResourcesAc {
    type Optioned = VolumeNodeResourcesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::storage::v1::VolumeNodeResources {
    fn into_optioned(self) -> VolumeNodeResourcesAc {
        VolumeNodeResourcesAc {
            count: self.count,
        }
    }
    fn try_from_optioned(value: VolumeNodeResourcesAc) -> Result<Self, crate::Error> {
        Ok(Self { count: value.count })
    }
    fn merge(&mut self, other: VolumeNodeResourcesAc) -> Result<(), crate::Error> {
        self.count = other.count;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::storage::v1::VolumeNodeResources>
for VolumeNodeResourcesAc {
    fn from_optionable(
        value: k8s_openapi027::api::storage::v1::VolumeNodeResources,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::storage::v1::VolumeNodeResources, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::storage::v1::VolumeNodeResources,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
