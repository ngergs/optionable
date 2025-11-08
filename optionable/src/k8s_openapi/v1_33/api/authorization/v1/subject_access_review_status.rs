#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct SubjectAccessReviewStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denied: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_error: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::authorization::v1::SubjectAccessReviewStatus {
    type Optioned = SubjectAccessReviewStatusAc;
}
#[automatically_derived]
impl crate::Optionable for SubjectAccessReviewStatusAc {
    type Optioned = SubjectAccessReviewStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::SubjectAccessReviewStatus {
    fn into_optioned(self) -> SubjectAccessReviewStatusAc {
        SubjectAccessReviewStatusAc {
            allowed: Some(self.allowed),
            denied: crate::OptionableConvert::into_optioned(self.denied),
            evaluation_error: crate::OptionableConvert::into_optioned(
                self.evaluation_error,
            ),
            reason: crate::OptionableConvert::into_optioned(self.reason),
        }
    }
    fn try_from_optioned(
        value: SubjectAccessReviewStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            allowed: value
                .allowed
                .ok_or(crate::Error {
                    missing_field: "allowed",
                })?,
            denied: crate::OptionableConvert::try_from_optioned(value.denied)?,
            evaluation_error: crate::OptionableConvert::try_from_optioned(
                value.evaluation_error,
            )?,
            reason: crate::OptionableConvert::try_from_optioned(value.reason)?,
        })
    }
    fn merge(&mut self, other: SubjectAccessReviewStatusAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.allowed {
            self.allowed = other_value;
        }
        crate::OptionableConvert::merge(&mut self.denied, other.denied)?;
        crate::OptionableConvert::merge(
            &mut self.evaluation_error,
            other.evaluation_error,
        )?;
        crate::OptionableConvert::merge(&mut self.reason, other.reason)?;
        Ok(())
    }
}
