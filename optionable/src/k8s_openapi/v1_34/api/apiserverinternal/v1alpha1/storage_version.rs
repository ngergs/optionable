#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageVersionAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionSpec as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<
        <::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersionStatus as crate::Optionable>::Optioned,
    >,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    phantom: std::marker::PhantomData<StorageVersionAc>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersion {
    type Optioned = StorageVersionAc;
}
#[automatically_derived]
impl crate::Optionable for StorageVersionAc {
    type Optioned = StorageVersionAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersion {
    fn into_optioned(self) -> StorageVersionAc {
        StorageVersionAc {
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
            status: Some(crate::OptionableConvert::into_optioned(self.status)),
        }
    }
    fn try_from_optioned(
        value: StorageVersionAc,
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
        other: StorageVersionAc,
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
impl k8s_openapi::Resource for StorageVersionAc {
    const API_VERSION: &'static str = "internal.apiserver.k8s.io/v1alpha1";
    const GROUP: &'static str = "internal.apiserver.k8s.io";
    const KIND: &'static str = "StorageVersion";
    const VERSION: &'static str = "v1alpha1";
    const URL_PATH_SEGMENT: &'static str = "storageversions";
    type Scope = k8s_openapi::ClusterResourceScope;
}
impl k8s_openapi::Metadata for StorageVersionAc {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
