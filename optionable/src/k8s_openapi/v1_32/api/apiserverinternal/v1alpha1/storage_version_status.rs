#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StorageVersionStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_encoding_version: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<
            ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionCondition,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_versions: <Option<
        std::vec::Vec<
            ::k8s_openapi::api::apiserverinternal::v1alpha1::ServerStorageVersion,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionStatus {
    type Optioned = StorageVersionStatusAc;
}
#[automatically_derived]
impl crate::Optionable for StorageVersionStatusAc {
    type Optioned = StorageVersionStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionStatus {
    fn into_optioned(self) -> StorageVersionStatusAc {
        StorageVersionStatusAc {
            common_encoding_version: crate::OptionableConvert::into_optioned(
                self.common_encoding_version,
            ),
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            storage_versions: crate::OptionableConvert::into_optioned(
                self.storage_versions,
            ),
        }
    }
    fn try_from_optioned(value: StorageVersionStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            common_encoding_version: crate::OptionableConvert::try_from_optioned(
                value.common_encoding_version,
            )?,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            storage_versions: crate::OptionableConvert::try_from_optioned(
                value.storage_versions,
            )?,
        })
    }
    fn merge(&mut self, other: StorageVersionStatusAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.common_encoding_version,
            other.common_encoding_version,
        )?;
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
    ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionStatus,
> for StorageVersionStatusAc {
    fn from_optionable(
        value: ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
