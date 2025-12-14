#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StatusCauseAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusCause {
    type Optioned = StatusCauseAc;
}
#[automatically_derived]
impl crate::Optionable for StatusCauseAc {
    type Optioned = StatusCauseAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusCause {
    fn into_optioned(self) -> StatusCauseAc {
        StatusCauseAc {
            field: crate::OptionableConvert::into_optioned(self.field),
            message: crate::OptionableConvert::into_optioned(self.message),
            reason: crate::OptionableConvert::into_optioned(self.reason),
        }
    }
    fn try_from_optioned(value: StatusCauseAc) -> Result<Self, crate::Error> {
        Ok(Self {
            field: crate::OptionableConvert::try_from_optioned(value.field)?,
            message: crate::OptionableConvert::try_from_optioned(value.message)?,
            reason: crate::OptionableConvert::try_from_optioned(value.reason)?,
        })
    }
    fn merge(&mut self, other: StatusCauseAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.field, other.field)?;
        crate::OptionableConvert::merge(&mut self.message, other.message)?;
        crate::OptionableConvert::merge(&mut self.reason, other.reason)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusCause,
> for StatusCauseAc {
    fn from_optionable(
        value: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusCause,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusCause,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusCause,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
