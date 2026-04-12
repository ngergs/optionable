#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ServiceAccountTokenProjection represents a projected service account token volume. This projection can be used to insert a service account token into the pods runtime filesystem for use against APIs (Kubernetes API Server or otherwise).
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServiceAccountTokenProjectionAc {
    /// audience is the intended audience of the token. A recipient of a token must identify itself with an identifier specified in the audience of the token, and otherwise should reject the token. The audience defaults to the identifier of the apiserver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<std::string::String>,
    /// expirationSeconds is the requested duration of validity of the service account token. As the token approaches expiration, the kubelet volume plugin will proactively rotate the service account token. The kubelet will start trying to rotate the token if the token is older than 80 percent of its time to live or if the token is older than 24 hours.Defaults to 1 hour and must be at least 10 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: Option<i64>,
    /// path is the path relative to the mount point of the file to project the token into.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ServiceAccountTokenProjection {
    type Optioned = ServiceAccountTokenProjectionAc;
}
#[automatically_derived]
impl crate::Optionable for ServiceAccountTokenProjectionAc {
    type Optioned = ServiceAccountTokenProjectionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::ServiceAccountTokenProjection {
    fn into_optioned(self) -> ServiceAccountTokenProjectionAc {
        ServiceAccountTokenProjectionAc {
            audience: self.audience,
            expiration_seconds: self.expiration_seconds,
            path: Some(self.path),
        }
    }
    fn try_from_optioned(
        value: ServiceAccountTokenProjectionAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            audience: value.audience,
            expiration_seconds: value.expiration_seconds,
            path: value
                .path
                .ok_or(crate::Error {
                    missing_field: "path",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: ServiceAccountTokenProjectionAc,
    ) -> Result<(), crate::Error> {
        if self.audience.is_none() {
            self.audience = crate::OptionableConvert::try_from_optioned(other.audience)?;
        } else if let Some(self_value) = self.audience.as_mut()
            && let Some(other_value) = other.audience
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.expiration_seconds.is_none() {
            self.expiration_seconds = crate::OptionableConvert::try_from_optioned(
                other.expiration_seconds,
            )?;
        } else if let Some(self_value) = self.expiration_seconds.as_mut()
            && let Some(other_value) = other.expiration_seconds
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.path {
            self.path = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ServiceAccountTokenProjection>
for ServiceAccountTokenProjectionAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ServiceAccountTokenProjection,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::ServiceAccountTokenProjection,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ServiceAccountTokenProjection,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
