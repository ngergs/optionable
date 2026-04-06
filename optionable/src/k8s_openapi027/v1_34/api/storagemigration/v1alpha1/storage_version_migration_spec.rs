#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StorageVersionMigrationSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_token: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<
        <::k8s_openapi027::api::storagemigration::v1alpha1::GroupVersionResource as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::storagemigration::v1alpha1::StorageVersionMigrationSpec {
    type Optioned = StorageVersionMigrationSpecAc;
}
#[automatically_derived]
impl crate::Optionable for StorageVersionMigrationSpecAc {
    type Optioned = StorageVersionMigrationSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::storagemigration::v1alpha1::StorageVersionMigrationSpec {
    fn into_optioned(self) -> StorageVersionMigrationSpecAc {
        StorageVersionMigrationSpecAc {
            continue_token: crate::OptionableConvert::into_optioned(self.continue_token),
            resource: Some(crate::OptionableConvert::into_optioned(self.resource)),
        }
    }
    fn try_from_optioned(
        value: StorageVersionMigrationSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            continue_token: crate::OptionableConvert::try_from_optioned(
                value.continue_token,
            )?,
            resource: crate::OptionableConvert::try_from_optioned(
                value
                    .resource
                    .ok_or(crate::Error {
                        missing_field: "resource",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: StorageVersionMigrationSpecAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.continue_token, other.continue_token)?;
        if let Some(other_value) = other.resource {
            crate::OptionableConvert::merge(&mut self.resource, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::storagemigration::v1alpha1::StorageVersionMigrationSpec,
> for StorageVersionMigrationSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::storagemigration::v1alpha1::StorageVersionMigrationSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::storagemigration::v1alpha1::StorageVersionMigrationSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::storagemigration::v1alpha1::StorageVersionMigrationSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
