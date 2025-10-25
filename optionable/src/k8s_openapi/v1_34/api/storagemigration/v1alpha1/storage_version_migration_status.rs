pub struct StorageVersionMigrationStatusOpt {
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::storagemigration::v1alpha1::MigrationCondition>,
    > as crate::Optionable>::Optioned,
    pub resource_version: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::storagemigration::v1alpha1::StorageVersionMigrationStatus {
    type Optioned = StorageVersionMigrationStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for StorageVersionMigrationStatusOpt {
    type Optioned = StorageVersionMigrationStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::storagemigration::v1alpha1::StorageVersionMigrationStatus {
    fn into_optioned(self) -> StorageVersionMigrationStatusOpt {
        StorageVersionMigrationStatusOpt {
            conditions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::storagemigration::v1alpha1::MigrationCondition,
                >,
            > as crate::OptionableConvert>::into_optioned(self.conditions),
            resource_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.resource_version),
        }
    }
    fn try_from_optioned(
        value: StorageVersionMigrationStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::storagemigration::v1alpha1::MigrationCondition,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.conditions)?,
            resource_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.resource_version)?,
        })
    }
    fn merge(
        &mut self,
        other: StorageVersionMigrationStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<
                ::k8s_openapi::api::storagemigration::v1alpha1::MigrationCondition,
            >,
        > as crate::OptionableConvert>::merge(&mut self.conditions, other.conditions)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.resource_version,
            other.resource_version,
        )?;
        Ok(())
    }
}
