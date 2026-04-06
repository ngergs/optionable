#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// API server instances report the versions they can decode and the version they encode objects to when persisting objects in the backend.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StorageVersionStatusAc {
    /// If all API server instances agree on the same encoding storage version, then this field is set to that version. Otherwise this field is left empty. API servers should finish updating its storageVersionStatus entry before serving write operations, so that this field will be in sync with the reality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_encoding_version: Option<std::string::String>,
    /// The latest available observations of the storageVersion's state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::apiserverinternal::v1alpha1::StorageVersionCondition as crate::Optionable>::Optioned,
        >,
    >,
    /// The reported versions per API server instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_versions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::apiserverinternal::v1alpha1::ServerStorageVersion as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::apiserverinternal::v1alpha1::StorageVersionStatus {
    type Optioned = StorageVersionStatusAc;
}
#[automatically_derived]
impl crate::Optionable for StorageVersionStatusAc {
    type Optioned = StorageVersionStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::apiserverinternal::v1alpha1::StorageVersionStatus {
    fn into_optioned(self) -> StorageVersionStatusAc {
        StorageVersionStatusAc {
            common_encoding_version: self.common_encoding_version,
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            storage_versions: crate::OptionableConvert::into_optioned(
                self.storage_versions,
            ),
        }
    }
    fn try_from_optioned(value: StorageVersionStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            common_encoding_version: value.common_encoding_version,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            storage_versions: crate::OptionableConvert::try_from_optioned(
                value.storage_versions,
            )?,
        })
    }
    fn merge(&mut self, other: StorageVersionStatusAc) -> Result<(), crate::Error> {
        self.common_encoding_version = other.common_encoding_version;
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        crate::OptionableConvert::merge(
            &mut self.storage_versions,
            other.storage_versions,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::apiserverinternal::v1alpha1::StorageVersionStatus,
> for StorageVersionStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::apiserverinternal::v1alpha1::StorageVersionStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::apiserverinternal::v1alpha1::StorageVersionStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apiserverinternal::v1alpha1::StorageVersionStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
