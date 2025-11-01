pub struct StorageVersionOpt {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub spec: Option<
        <::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionSpec as crate::Optionable>::Optioned,
    >,
    pub status: Option<
        <::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionStatus as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersion {
    type Optioned = StorageVersionOpt;
}
#[automatically_derived]
impl crate::Optionable for StorageVersionOpt {
    type Optioned = StorageVersionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersion {
    fn into_optioned(self) -> StorageVersionOpt {
        StorageVersionOpt {
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
            status: Some(crate::OptionableConvert::into_optioned(self.status)),
        }
    }
    fn try_from_optioned(
        value: StorageVersionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::optionable::Error {
                        missing_field: "spec",
                    })?,
            )?,
            status: crate::OptionableConvert::try_from_optioned(
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
        self.metadata = other.metadata;
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        if let Some(other_value) = other.status {
            crate::OptionableConvert::merge(&mut self.status, other_value)?;
        }
        Ok(())
    }
}
