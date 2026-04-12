#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// SubjectAccessReviewStatus
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SubjectAccessReviewStatusAc {
    /// Allowed is required. True if the action would be allowed, false otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed: Option<bool>,
    /// Denied is optional. True if the action would be denied, otherwise false. If both allowed is false and denied is false, then the authorizer has no opinion on whether to authorize the action. Denied may not be true if Allowed is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denied: Option<bool>,
    /// EvaluationError is an indication that some error occurred during the authorization check. It is entirely possible to get an error and be able to continue determine authorization status in spite of it. For instance, RBAC can be missing a role, but enough roles are still present and bound to reason about the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_error: Option<std::string::String>,
    /// Reason is optional.  It indicates why a request was allowed or denied.
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
        if other.denied.is_some() {
            self.denied = other.denied;
        }
        if other.evaluation_error.is_some() {
            self.evaluation_error = other.evaluation_error;
        }
        if other.reason.is_some() {
            self.reason = other.reason;
        }
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
