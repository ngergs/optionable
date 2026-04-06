#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StatusDetailsAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causes: Option<
        std::vec::Vec<
            <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::StatusCause as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after_seconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::StatusDetails {
    type Optioned = StatusDetailsAc;
}
#[automatically_derived]
impl crate::Optionable for StatusDetailsAc {
    type Optioned = StatusDetailsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::StatusDetails {
    fn into_optioned(self) -> StatusDetailsAc {
        StatusDetailsAc {
            causes: crate::OptionableConvert::into_optioned(self.causes),
            group: self.group,
            kind: self.kind,
            name: self.name,
            retry_after_seconds: self.retry_after_seconds,
            uid: self.uid,
        }
    }
    fn try_from_optioned(value: StatusDetailsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            causes: crate::OptionableConvert::try_from_optioned(value.causes)?,
            group: value.group,
            kind: value.kind,
            name: value.name,
            retry_after_seconds: value.retry_after_seconds,
            uid: value.uid,
        })
    }
    fn merge(&mut self, other: StatusDetailsAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.causes, other.causes)?;
        self.group = other.group;
        self.kind = other.kind;
        self.name = other.name;
        self.retry_after_seconds = other.retry_after_seconds;
        self.uid = other.uid;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apimachinery::pkg::apis::meta::v1::StatusDetails,
> for StatusDetailsAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::StatusDetails,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::StatusDetails,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::StatusDetails,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
