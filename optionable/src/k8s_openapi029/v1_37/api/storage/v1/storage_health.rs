#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// StorageHealth contains storage backend health reported by a CSI driver on a node.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StorageHealthAc {
    /// healthConditions are the adverse storage backend conditions reported by the CSI driver. At most 16 conditions may be reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_conditions: Option<
        std::vec::Vec<
            <::k8s_openapi029::api::storage::v1::StorageHealthCondition as crate::Optionable>::Optioned,
        >,
    >,
    /// name is the CSI driver name, matching CSINodeDriver.name.
    pub name: std::string::String,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::storage::v1::StorageHealth {
    type Optioned = StorageHealthAc;
}
#[automatically_derived]
impl crate::Optionable for StorageHealthAc {
    type Optioned = StorageHealthAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi029::api::storage::v1::StorageHealth {
    fn into_optioned(self) -> StorageHealthAc {
        StorageHealthAc {
            health_conditions: crate::OptionableConvert::into_optioned(
                self.health_conditions,
            ),
            name: self.name,
        }
    }
    fn try_from_optioned(value: StorageHealthAc) -> Result<Self, crate::Error> {
        Ok(Self {
            health_conditions: crate::OptionableConvert::try_from_optioned(
                value.health_conditions,
            )?,
            name: value.name,
        })
    }
    fn merge(&mut self, other: StorageHealthAc) -> Result<(), crate::Error> {
        if self.health_conditions.is_none() {
            self.health_conditions = crate::OptionableConvert::try_from_optioned(
                other.health_conditions,
            )?;
        } else if let Some(self_value) = self.health_conditions.as_mut()
            && let Some(other_value) = other.health_conditions
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        self.name = other.name;
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi029::api::storage::v1::StorageHealth {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.name == other.name
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::storage::v1::StorageHealth>
for StorageHealthAc {
    fn from_optionable(value: k8s_openapi029::api::storage::v1::StorageHealth) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi029::api::storage::v1::StorageHealth, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::storage::v1::StorageHealth,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for StorageHealthAc {
    fn merge_from(&mut self, other: Self) {
        self.health_conditions = other.health_conditions;
        k8s_openapi029::DeepMerge::merge_from(&mut self.name, other.name);
    }
}
impl crate::merge::MapKeysEq for StorageHealthAc {
    fn keys_eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
