pub struct StorageVersionStatusOpt {
    pub common_encoding_version: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub conditions: <Option<
        std::vec::Vec<
            ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionCondition,
        >,
    > as crate::Optionable>::Optioned,
    pub storage_versions: <Option<
        std::vec::Vec<
            ::k8s_openapi::api::apiserverinternal::v1alpha1::ServerStorageVersion,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionStatus {
    type Optioned = StorageVersionStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for StorageVersionStatusOpt {
    type Optioned = StorageVersionStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionStatus {
    fn into_optioned(self) -> StorageVersionStatusOpt {
        StorageVersionStatusOpt {
            common_encoding_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.common_encoding_version),
            conditions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionCondition,
                >,
            > as crate::OptionableConvert>::into_optioned(self.conditions),
            storage_versions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::apiserverinternal::v1alpha1::ServerStorageVersion,
                >,
            > as crate::OptionableConvert>::into_optioned(self.storage_versions),
        }
    }
    fn try_from_optioned(
        value: StorageVersionStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            common_encoding_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.common_encoding_version,
            )?,
            conditions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionCondition,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.conditions)?,
            storage_versions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::apiserverinternal::v1alpha1::ServerStorageVersion,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.storage_versions)?,
        })
    }
    fn merge(
        &mut self,
        other: StorageVersionStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.common_encoding_version,
            other.common_encoding_version,
        )?;
        <Option<
            std::vec::Vec<
                ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionCondition,
            >,
        > as crate::OptionableConvert>::merge(&mut self.conditions, other.conditions)?;
        <Option<
            std::vec::Vec<
                ::k8s_openapi::api::apiserverinternal::v1alpha1::ServerStorageVersion,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.storage_versions,
            other.storage_versions,
        )?;
        Ok(())
    }
}
