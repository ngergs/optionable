#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// VolumeError captures an error encountered during a volume operation.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VolumeErrorAc {
    /// errorCode is a numeric gRPC code representing the error encountered during Attach or Detach operations.
    ///
    /// This is an optional, alpha field that requires the MutableCSINodeAllocatableCount feature gate being enabled to be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    /// message represents the error encountered during Attach or Detach operation. This string may be logged, so it should not contain sensitive information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// time represents the time the error was encountered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::storage::v1::VolumeError {
    type Optioned = VolumeErrorAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeErrorAc {
    type Optioned = VolumeErrorAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::storage::v1::VolumeError {
    fn into_optioned(self) -> VolumeErrorAc {
        VolumeErrorAc {
            error_code: self.error_code,
            message: self.message,
            time: crate::OptionableConvert::into_optioned(self.time),
        }
    }
    fn try_from_optioned(value: VolumeErrorAc) -> Result<Self, crate::Error> {
        Ok(Self {
            error_code: value.error_code,
            message: value.message,
            time: crate::OptionableConvert::try_from_optioned(value.time)?,
        })
    }
    fn merge(&mut self, other: VolumeErrorAc) -> Result<(), crate::Error> {
        if other.error_code.is_some() {
            self.error_code = other.error_code;
        }
        if other.message.is_some() {
            self.message = other.message;
        }
        crate::OptionableConvert::merge(&mut self.time, other.time)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::storage::v1::VolumeError>
for VolumeErrorAc {
    fn from_optionable(value: k8s_openapi027::api::storage::v1::VolumeError) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::storage::v1::VolumeError, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::storage::v1::VolumeError,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
