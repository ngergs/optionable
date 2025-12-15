#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StatusAc {
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_api_version",
        deserialize_with = "crate::k8s_openapi::deserialize_api_version"
    )]
    pub api_version: std::marker::PhantomData<Self>,
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_kind",
        deserialize_with = "crate::k8s_openapi::deserialize_kind"
    )]
    pub kind: std::marker::PhantomData<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusDetails,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ListMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Status {
    type Optioned = StatusAc;
}
#[automatically_derived]
impl crate::Optionable for StatusAc {
    type Optioned = StatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Status {
    fn into_optioned(self) -> StatusAc {
        StatusAc {
            api_version: Default::default(),
            kind: Default::default(),
            code: crate::OptionableConvert::into_optioned(self.code),
            details: crate::OptionableConvert::into_optioned(self.details),
            message: crate::OptionableConvert::into_optioned(self.message),
            metadata: self.metadata,
            reason: crate::OptionableConvert::into_optioned(self.reason),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(value: StatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            code: crate::OptionableConvert::try_from_optioned(value.code)?,
            details: crate::OptionableConvert::try_from_optioned(value.details)?,
            message: crate::OptionableConvert::try_from_optioned(value.message)?,
            metadata: value.metadata,
            reason: crate::OptionableConvert::try_from_optioned(value.reason)?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(&mut self, other: StatusAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.code, other.code)?;
        crate::OptionableConvert::merge(&mut self.details, other.details)?;
        crate::OptionableConvert::merge(&mut self.message, other.message)?;
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.reason, other.reason)?;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::apimachinery::pkg::apis::meta::v1::Status>
for StatusAc {
    fn from_optionable(
        value: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Status,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::apimachinery::pkg::apis::meta::v1::Status, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Status,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi::Resource for StatusAc {
    const API_VERSION: &'static str = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::Status as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::Status as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::Status as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::Status as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::Status as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::Status as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for StatusAc {
    type Ty = <::k8s_openapi::apimachinery::pkg::apis::meta::v1::Status as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_statusac() {
    crate::testutil::roundtrip_test::<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Status,
    >();
}
