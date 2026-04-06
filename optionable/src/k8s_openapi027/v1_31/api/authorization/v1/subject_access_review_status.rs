#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SubjectAccessReviewStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_error: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::authorization::v1::SubjectAccessReviewStatus {
    type Optioned = SubjectAccessReviewStatusAc;
}
#[automatically_derived]
impl crate::Optionable for SubjectAccessReviewStatusAc {
    type Optioned = SubjectAccessReviewStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::authorization::v1::SubjectAccessReviewStatus {
    fn into_optioned(self) -> SubjectAccessReviewStatusAc {
        SubjectAccessReviewStatusAc {
            allowed: Some(self.allowed),
            denied: self.denied,
            evaluation_error: self.evaluation_error,
            reason: self.reason,
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
            denied: value.denied,
            evaluation_error: value.evaluation_error,
            reason: value.reason,
        })
    }
    fn merge(&mut self, other: SubjectAccessReviewStatusAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.allowed {
            self.allowed = other_value;
        }
        self.denied = other.denied;
        self.evaluation_error = other.evaluation_error;
        self.reason = other.reason;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::authorization::v1::SubjectAccessReviewStatus,
> for SubjectAccessReviewStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::authorization::v1::SubjectAccessReviewStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::authorization::v1::SubjectAccessReviewStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::authorization::v1::SubjectAccessReviewStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
