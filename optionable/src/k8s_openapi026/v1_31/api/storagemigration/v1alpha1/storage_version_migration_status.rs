#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StorageVersionMigrationStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<
            ::k8s_openapi026::api::storagemigration::v1alpha1::MigrationCondition,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_version: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::api::storagemigration::v1alpha1::StorageVersionMigrationStatus {
    type Optioned = StorageVersionMigrationStatusAc;
}
#[automatically_derived]
impl crate::Optionable for StorageVersionMigrationStatusAc {
    type Optioned = StorageVersionMigrationStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::storagemigration::v1alpha1::StorageVersionMigrationStatus {
    fn into_optioned(self) -> StorageVersionMigrationStatusAc {
        StorageVersionMigrationStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            resource_version: crate::OptionableConvert::into_optioned(
                self.resource_version,
            ),
        }
    }
    fn try_from_optioned(
        value: StorageVersionMigrationStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            resource_version: crate::OptionableConvert::try_from_optioned(
                value.resource_version,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: StorageVersionMigrationStatusAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        crate::OptionableConvert::merge(
            &mut self.resource_version,
            other.resource_version,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::api::storagemigration::v1alpha1::StorageVersionMigrationStatus,
> for StorageVersionMigrationStatusAc {
    fn from_optionable(
        value: k8s_openapi026::api::storagemigration::v1alpha1::StorageVersionMigrationStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::storagemigration::v1alpha1::StorageVersionMigrationStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::storagemigration::v1alpha1::StorageVersionMigrationStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
