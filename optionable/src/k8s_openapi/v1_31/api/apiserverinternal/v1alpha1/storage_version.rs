#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
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
        deserialize_with = "crate::k8s_openapi::deserialize_api_envelope"
    )]
    pub phantom: std::marker::PhantomData<Self>,
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
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersion {
    fn into_optioned(self) -> StorageVersionAc {
        StorageVersionAc {
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
            status: Some(crate::OptionableConvert::into_optioned(self.status)),
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(value: StorageVersionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::Error {
                        missing_field: "spec",
                    })?,
            )?,
            status: crate::OptionableConvert::try_from_optioned(
                value
                    .status
                    .ok_or(crate::Error {
                        missing_field: "status",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: StorageVersionAc) -> Result<(), crate::Error> {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersion,
> for StorageVersionAc {
    fn from_optionable(
        value: ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersion,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersion,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersion,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi::Resource for StorageVersionAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersion as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersion as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersion as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersion as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersion as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersion as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for StorageVersionAc {
    type Ty = <::k8s_openapi::api::apiserverinternal::v1alpha1::StorageVersion as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
