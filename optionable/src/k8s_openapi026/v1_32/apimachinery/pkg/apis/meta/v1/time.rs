#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TimeAc(
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Option<
        <::k8s_openapi026::chrono::DateTime<
            ::k8s_openapi026::chrono::Utc,
        > as crate::Optionable>::Optioned,
    >,
);
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::apimachinery::pkg::apis::meta::v1::Time {
    type Optioned = TimeAc;
}
#[automatically_derived]
impl crate::Optionable for TimeAc {
    type Optioned = TimeAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::apimachinery::pkg::apis::meta::v1::Time {
    fn into_optioned(self) -> TimeAc {
        TimeAc(Some(crate::OptionableConvert::into_optioned(self.0)))
    }
    fn try_from_optioned(value: TimeAc) -> Result<Self, crate::Error> {
        Ok(
            Self(
                crate::OptionableConvert::try_from_optioned(
                    value.0.ok_or(crate::Error { missing_field: "0" })?,
                )?,
            ),
        )
    }
    fn merge(&mut self, other: TimeAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.0 {
            crate::OptionableConvert::merge(&mut self.0, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::apimachinery::pkg::apis::meta::v1::Time>
for TimeAc {
    fn from_optionable(
        value: k8s_openapi026::apimachinery::pkg::apis::meta::v1::Time,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::apimachinery::pkg::apis::meta::v1::Time, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::apimachinery::pkg::apis::meta::v1::Time,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
