#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct StorageVersionMigrationSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_token: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<
        <::k8s_openapi::api::storagemigration::v1alpha1::GroupVersionResource as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::storagemigration::v1alpha1::StorageVersionMigrationSpec {
    type Optioned = StorageVersionMigrationSpecAc;
}
#[automatically_derived]
impl crate::Optionable for StorageVersionMigrationSpecAc {
    type Optioned = StorageVersionMigrationSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::storagemigration::v1alpha1::StorageVersionMigrationSpec {
    fn into_optioned(self) -> StorageVersionMigrationSpecAc {
        StorageVersionMigrationSpecAc {
            continue_token: crate::OptionableConvert::into_optioned(self.continue_token),
            resource: Some(crate::OptionableConvert::into_optioned(self.resource)),
        }
    }
    fn try_from_optioned(
        value: StorageVersionMigrationSpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            continue_token: crate::OptionableConvert::try_from_optioned(
                value.continue_token,
            )?,
            resource: crate::OptionableConvert::try_from_optioned(
                value
                    .resource
                    .ok_or(crate::optionable::Error {
                        missing_field: "resource",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: StorageVersionMigrationSpecAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.continue_token, other.continue_token)?;
        if let Some(other_value) = other.resource {
            crate::OptionableConvert::merge(&mut self.resource, other_value)?;
        }
        Ok(())
    }
}
