#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CSINodeStatus contains health and status information for storage on a node.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CSINodeStatusAc {
    /// storageHealth contains backend health reports for CSI drivers registered on the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_health: Option<
        std::vec::Vec<
            <::k8s_openapi029::api::storage::v1::StorageHealth as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::storage::v1::CSINodeStatus {
    type Optioned = CSINodeStatusAc;
}
#[automatically_derived]
impl crate::Optionable for CSINodeStatusAc {
    type Optioned = CSINodeStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi029::api::storage::v1::CSINodeStatus {
    fn into_optioned(self) -> CSINodeStatusAc {
        CSINodeStatusAc {
            storage_health: crate::OptionableConvert::into_optioned(self.storage_health),
        }
    }
    fn try_from_optioned(value: CSINodeStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            storage_health: crate::OptionableConvert::try_from_optioned(
                value.storage_health,
            )?,
        })
    }
    fn merge(&mut self, other: CSINodeStatusAc) -> Result<(), crate::Error> {
        if self.storage_health.is_none() {
            self.storage_health = crate::OptionableConvert::try_from_optioned(
                other.storage_health,
            )?;
        } else if let Some(self_value) = self.storage_health.as_mut()
            && let Some(other_value) = other.storage_health
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::storage::v1::CSINodeStatus>
for CSINodeStatusAc {
    fn from_optionable(value: k8s_openapi029::api::storage::v1::CSINodeStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi029::api::storage::v1::CSINodeStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::storage::v1::CSINodeStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for CSINodeStatusAc {
    fn merge_from(&mut self, other: Self) {
        crate::k8s_openapi::merge::merge_map_option_wrapped(
            &mut self.storage_health,
            other.storage_health,
        );
    }
}
