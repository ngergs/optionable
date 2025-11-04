#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
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
    fn try_from_optioned(
        value: StorageVersionStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
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
    fn merge(
        &mut self,
        other: StorageVersionStatusAc,
    ) -> Result<(), crate::optionable::Error> {
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
