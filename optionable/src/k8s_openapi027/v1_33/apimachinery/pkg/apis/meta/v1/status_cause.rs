#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// StatusCause provides more information about an api.Status failure, including cases when multiple errors are encountered.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StatusCauseAc {
    /// The field of the resource that has caused this error, as named by its JSON serialization. May include dot and postfix notation for nested attributes. Arrays are zero-indexed.  Fields may appear more than once in an array of causes due to fields having multiple errors. Optional.
    ///
    /// Examples:
    ///   "name" - the field "name" on the current resource
    ///   "items\[0\].name" - the field "name" on the first array entry in "items"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<std::string::String>,
    /// A human-readable description of the cause of the error.  This field may be presented as-is to a reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// A machine-readable description of the cause of the error. If this value is empty there is no information available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::StatusCause {
    type Optioned = StatusCauseAc;
}
#[automatically_derived]
impl crate::Optionable for StatusCauseAc {
    type Optioned = StatusCauseAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::StatusCause {
    fn into_optioned(self) -> StatusCauseAc {
        StatusCauseAc {
            field: self.field,
            message: self.message,
            reason: self.reason,
        }
    }
    fn try_from_optioned(value: StatusCauseAc) -> Result<Self, crate::Error> {
        Ok(Self {
            field: value.field,
            message: value.message,
            reason: value.reason,
        })
    }
    fn merge(&mut self, other: StatusCauseAc) -> Result<(), crate::Error> {
        if self.field.is_none() {
            self.field = other.field;
        }
        if let Some(other_value) = other.field {
            crate::OptionableConvert::merge(&mut self.field, other_value)?;
        }
        if self.message.is_none() {
            self.message = other.message;
        }
        if let Some(other_value) = other.message {
            crate::OptionableConvert::merge(&mut self.message, other_value)?;
        }
        if self.reason.is_none() {
            self.reason = other.reason;
        }
        if let Some(other_value) = other.reason {
            crate::OptionableConvert::merge(&mut self.reason, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apimachinery::pkg::apis::meta::v1::StatusCause,
> for StatusCauseAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::StatusCause,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::StatusCause,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::StatusCause,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
