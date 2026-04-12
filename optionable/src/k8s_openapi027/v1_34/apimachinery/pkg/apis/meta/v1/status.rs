#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Status is a return value for calls that don't return other objects.
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
    /// Suggested HTTP return code for this status, 0 if not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// Extended data associated with the reason.  Each reason may define its own extended details. This field is optional and the data returned is not guaranteed to conform to any schema except that defined by the reason type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::StatusDetails as crate::Optionable>::Optioned,
    >,
    /// A human-readable description of the status of this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ListMeta,
    /// A machine-readable description of why this operation is in the "Failure" status. If this value is empty there is no information available. A Reason clarifies an HTTP status code but does not override it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
    /// Status of the operation. One of: "Success" or "Failure". More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::apimachinery::pkg::apis::meta::v1::Status {
    type Optioned = StatusAc;
}
#[automatically_derived]
impl crate::Optionable for StatusAc {
    type Optioned = StatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::Status {
    fn into_optioned(self) -> StatusAc {
        StatusAc {
            api_version: Default::default(),
            kind: Default::default(),
            code: self.code,
            details: crate::OptionableConvert::into_optioned(self.details),
            message: self.message,
            metadata: self.metadata,
            reason: self.reason,
            status: self.status,
        }
    }
    fn try_from_optioned(value: StatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            code: value.code,
            details: crate::OptionableConvert::try_from_optioned(value.details)?,
            message: value.message,
            metadata: value.metadata,
            reason: value.reason,
            status: value.status,
        })
    }
    fn merge(&mut self, other: StatusAc) -> Result<(), crate::Error> {
        if other.code.is_some() {
            self.code = other.code;
        }
        crate::OptionableConvert::merge(&mut self.details, other.details)?;
        if other.message.is_some() {
            self.message = other.message;
        }
        self.metadata = other.metadata;
        if other.reason.is_some() {
            self.reason = other.reason;
        }
        if other.status.is_some() {
            self.status = other.status;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::apimachinery::pkg::apis::meta::v1::Status>
for StatusAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::Status,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::Status,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::Status,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for StatusAc {
    const API_VERSION: &'static str = <k8s_openapi027::apimachinery::pkg::apis::meta::v1::Status as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::apimachinery::pkg::apis::meta::v1::Status as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::apimachinery::pkg::apis::meta::v1::Status as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::apimachinery::pkg::apis::meta::v1::Status as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::apimachinery::pkg::apis::meta::v1::Status as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::apimachinery::pkg::apis::meta::v1::Status as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for StatusAc {
    type Ty = <k8s_openapi027::apimachinery::pkg::apis::meta::v1::Status as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_statusac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::Status,
    >();
}
