pub struct StorageVersionOpt {
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub spec: Option<
        <::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionSpec as crate::Optionable>::Optioned,
    >,
    pub status: Option<
        <::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionStatus as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::apiserverinternal::v1alpha1::storage_version::StorageVersion {
    type Optioned = StorageVersionOpt;
}
#[automatically_derived]
impl crate::Optionable for StorageVersionOpt {
    type Optioned = StorageVersionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::apiserverinternal::v1alpha1::storage_version::StorageVersion {
    fn into_optioned(self) -> StorageVersionOpt {
        StorageVersionOpt {
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            spec: Some(
                <::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionSpec as crate::OptionableConvert>::into_optioned(
                    self.spec,
                ),
            ),
            status: Some(
                <::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionStatus as crate::OptionableConvert>::into_optioned(
                    self.status,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: StorageVersionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            spec: <::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionSpec as crate::OptionableConvert>::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::optionable::Error {
                        missing_field: "spec",
                    })?,
            )?,
            status: <::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionStatus as crate::OptionableConvert>::try_from_optioned(
                value
                    .status
                    .ok_or(crate::optionable::Error {
                        missing_field: "status",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: StorageVersionOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        if let Some(other_value) = other.spec {
            <::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionSpec as crate::OptionableConvert>::merge(
                &mut self.spec,
                other_value,
            )?;
        }
        if let Some(other_value) = other.status {
            <::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionStatus as crate::OptionableConvert>::merge(
                &mut self.status,
                other_value,
            )?;
        }
        Ok(())
    }
}
