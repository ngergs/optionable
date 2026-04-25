#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Status of the storage version migration.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StorageVersionMigrationStatusAc {
    /// The latest available observations of the migration's current state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::storagemigration::v1alpha1::MigrationCondition as crate::Optionable>::Optioned,
        >,
    >,
    /// ResourceVersion to compare with the GC cache for performing the migration. This is the current resource version of given group, version and resource when kube-controller-manager first observes this StorageVersionMigration resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::storagemigration::v1alpha1::StorageVersionMigrationStatus {
    type Optioned = StorageVersionMigrationStatusAc;
}
#[automatically_derived]
impl crate::Optionable for StorageVersionMigrationStatusAc {
    type Optioned = StorageVersionMigrationStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::storagemigration::v1alpha1::StorageVersionMigrationStatus {
    fn into_optioned(self) -> StorageVersionMigrationStatusAc {
        StorageVersionMigrationStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            resource_version: self.resource_version,
        }
    }
    fn try_from_optioned(
        value: StorageVersionMigrationStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            resource_version: value.resource_version,
        })
    }
    fn merge(
        &mut self,
        other: StorageVersionMigrationStatusAc,
    ) -> Result<(), crate::Error> {
        if self.conditions.is_none() {
            self.conditions = crate::OptionableConvert::try_from_optioned(
                other.conditions,
            )?;
        } else if let Some(self_value) = self.conditions.as_mut()
            && let Some(other_value) = other.conditions
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.resource_version.is_none() {
            self.resource_version = crate::OptionableConvert::try_from_optioned(
                other.resource_version,
            )?;
        } else if let Some(self_value) = self.resource_version.as_mut()
            && let Some(other_value) = other.resource_version
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::storagemigration::v1alpha1::StorageVersionMigrationStatus,
> for StorageVersionMigrationStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::storagemigration::v1alpha1::StorageVersionMigrationStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::storagemigration::v1alpha1::StorageVersionMigrationStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::storagemigration::v1alpha1::StorageVersionMigrationStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for StorageVersionMigrationStatusAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.conditions, other.conditions);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.resource_version,
            other.resource_version,
        );
    }
}
