#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionMigrationStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::storagemigration::v1alpha1::MigrationCondition>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_version: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::storagemigration::v1alpha1::StorageVersionMigrationStatus {
    type Optioned = StorageVersionMigrationStatusAc;
}
#[automatically_derived]
impl crate::Optionable for StorageVersionMigrationStatusAc {
    type Optioned = StorageVersionMigrationStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::storagemigration::v1alpha1::StorageVersionMigrationStatus {
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
    ) -> Result<Self, crate::optionable::Error> {
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
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        crate::OptionableConvert::merge(
            &mut self.resource_version,
            other.resource_version,
        )?;
        Ok(())
    }
}
