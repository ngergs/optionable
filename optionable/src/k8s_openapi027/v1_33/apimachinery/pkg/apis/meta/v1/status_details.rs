#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// StatusDetails is a set of additional properties that MAY be set by the server to provide additional information about a response. The Reason field of a Status object defines what attributes will be set. Clients must ignore fields that do not match the defined type of each attribute, and should assume that any attribute may be empty, invalid, or under defined.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StatusDetailsAc {
    /// The Causes array includes more details associated with the StatusReason failure. Not all StatusReasons may provide detailed causes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causes: Option<
        std::vec::Vec<
            <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::StatusCause as crate::Optionable>::Optioned,
        >,
    >,
    /// The group attribute of the resource associated with the status StatusReason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<std::string::String>,
    /// The kind attribute of the resource associated with the status StatusReason. On some operations may differ from the requested resource Kind. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<std::string::String>,
    /// The name attribute of the resource associated with the status StatusReason (when there is a single name which can be described).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// If specified, the time in seconds before the operation should be retried. Some errors may indicate the client must take an alternate action - for those errors this field may indicate how long to wait before taking the alternate action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after_seconds: Option<i32>,
    /// UID of the resource. (when there is a single resource which can be described). More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names#uids
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
        if other.group.is_some() {
            self.group = other.group;
        }
        if other.kind.is_some() {
            self.kind = other.kind;
        }
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.retry_after_seconds.is_some() {
            self.retry_after_seconds = other.retry_after_seconds;
        }
        if other.uid.is_some() {
            self.uid = other.uid;
        }
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
