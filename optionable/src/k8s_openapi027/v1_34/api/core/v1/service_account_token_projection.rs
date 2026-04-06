#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServiceAccountTokenProjectionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: Option<i64>,
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
        self.audience = other.audience;
        self.expiration_seconds = other.expiration_seconds;
        if let Some(other_value) = other.path {
            self.path = other_value;
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
