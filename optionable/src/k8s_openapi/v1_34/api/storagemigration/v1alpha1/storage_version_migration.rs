#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    kube::Resource
)]
#[resource(inherit = StorageVersionMigration)]
pub struct StorageVersionMigrationAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: <Option<
        ::k8s_openapi::api::storagemigration::v1alpha1::StorageVersionMigrationSpec,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: <Option<
        ::k8s_openapi::api::storagemigration::v1alpha1::StorageVersionMigrationStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::storagemigration::v1alpha1::StorageVersionMigration {
    type Optioned = StorageVersionMigrationAc;
}
#[automatically_derived]
impl crate::Optionable for StorageVersionMigrationAc {
    type Optioned = StorageVersionMigrationAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::storagemigration::v1alpha1::StorageVersionMigration {
    fn into_optioned(self) -> StorageVersionMigrationAc {
        StorageVersionMigrationAc {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(
        value: StorageVersionMigrationAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(
        &mut self,
        other: StorageVersionMigrationAc,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::storagemigration::v1alpha1::StorageVersionMigration;
