#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct StatusDetailsAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causes: <Option<
        std::vec::Vec<::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusCause>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after_seconds: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusDetails {
    type Optioned = StatusDetailsAc;
}
#[automatically_derived]
impl crate::Optionable for StatusDetailsAc {
    type Optioned = StatusDetailsAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusDetails {
    fn into_optioned(self) -> StatusDetailsAc {
        StatusDetailsAc {
            causes: crate::OptionableConvert::into_optioned(self.causes),
            group: crate::OptionableConvert::into_optioned(self.group),
            kind: crate::OptionableConvert::into_optioned(self.kind),
            name: crate::OptionableConvert::into_optioned(self.name),
            retry_after_seconds: crate::OptionableConvert::into_optioned(
                self.retry_after_seconds,
            ),
            uid: crate::OptionableConvert::into_optioned(self.uid),
        }
    }
    fn try_from_optioned(value: StatusDetailsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            causes: crate::OptionableConvert::try_from_optioned(value.causes)?,
            group: crate::OptionableConvert::try_from_optioned(value.group)?,
            kind: crate::OptionableConvert::try_from_optioned(value.kind)?,
            name: crate::OptionableConvert::try_from_optioned(value.name)?,
            retry_after_seconds: crate::OptionableConvert::try_from_optioned(
                value.retry_after_seconds,
            )?,
            uid: crate::OptionableConvert::try_from_optioned(value.uid)?,
        })
    }
    fn merge(&mut self, other: StatusDetailsAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.causes, other.causes)?;
        crate::OptionableConvert::merge(&mut self.group, other.group)?;
        crate::OptionableConvert::merge(&mut self.kind, other.kind)?;
        crate::OptionableConvert::merge(&mut self.name, other.name)?;
        crate::OptionableConvert::merge(
            &mut self.retry_after_seconds,
            other.retry_after_seconds,
        )?;
        crate::OptionableConvert::merge(&mut self.uid, other.uid)?;
        Ok(())
    }
}
