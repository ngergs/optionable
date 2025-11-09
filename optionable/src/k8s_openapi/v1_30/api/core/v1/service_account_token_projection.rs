#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccountTokenProjectionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: <Option<i64> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ServiceAccountTokenProjection {
    type Optioned = ServiceAccountTokenProjectionAc;
}
#[automatically_derived]
impl crate::Optionable for ServiceAccountTokenProjectionAc {
    type Optioned = ServiceAccountTokenProjectionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::ServiceAccountTokenProjection {
    fn into_optioned(self) -> ServiceAccountTokenProjectionAc {
        ServiceAccountTokenProjectionAc {
            audience: crate::OptionableConvert::into_optioned(self.audience),
            expiration_seconds: crate::OptionableConvert::into_optioned(
                self.expiration_seconds,
            ),
            path: Some(crate::OptionableConvert::into_optioned(self.path)),
        }
    }
    fn try_from_optioned(
        value: ServiceAccountTokenProjectionAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            audience: crate::OptionableConvert::try_from_optioned(value.audience)?,
            expiration_seconds: crate::OptionableConvert::try_from_optioned(
                value.expiration_seconds,
            )?,
            path: crate::OptionableConvert::try_from_optioned(
                value
                    .path
                    .ok_or(crate::Error {
                        missing_field: "path",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ServiceAccountTokenProjectionAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.audience, other.audience)?;
        crate::OptionableConvert::merge(
            &mut self.expiration_seconds,
            other.expiration_seconds,
        )?;
        if let Some(other_value) = other.path {
            crate::OptionableConvert::merge(&mut self.path, other_value)?;
        }
        Ok(())
    }
}
