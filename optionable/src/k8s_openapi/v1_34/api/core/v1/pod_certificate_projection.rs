#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodCertificateProjectionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain_path: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_bundle_path: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_path: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_expiration_seconds: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PodCertificateProjection {
    type Optioned = PodCertificateProjectionAc;
}
#[automatically_derived]
impl crate::Optionable for PodCertificateProjectionAc {
    type Optioned = PodCertificateProjectionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::PodCertificateProjection {
    fn into_optioned(self) -> PodCertificateProjectionAc {
        PodCertificateProjectionAc {
            certificate_chain_path: crate::OptionableConvert::into_optioned(
                self.certificate_chain_path,
            ),
            credential_bundle_path: crate::OptionableConvert::into_optioned(
                self.credential_bundle_path,
            ),
            key_path: crate::OptionableConvert::into_optioned(self.key_path),
            key_type: Some(crate::OptionableConvert::into_optioned(self.key_type)),
            max_expiration_seconds: crate::OptionableConvert::into_optioned(
                self.max_expiration_seconds,
            ),
            signer_name: Some(crate::OptionableConvert::into_optioned(self.signer_name)),
        }
    }
    fn try_from_optioned(
        value: PodCertificateProjectionAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            certificate_chain_path: crate::OptionableConvert::try_from_optioned(
                value.certificate_chain_path,
            )?,
            credential_bundle_path: crate::OptionableConvert::try_from_optioned(
                value.credential_bundle_path,
            )?,
            key_path: crate::OptionableConvert::try_from_optioned(value.key_path)?,
            key_type: crate::OptionableConvert::try_from_optioned(
                value
                    .key_type
                    .ok_or(crate::Error {
                        missing_field: "key_type",
                    })?,
            )?,
            max_expiration_seconds: crate::OptionableConvert::try_from_optioned(
                value.max_expiration_seconds,
            )?,
            signer_name: crate::OptionableConvert::try_from_optioned(
                value
                    .signer_name
                    .ok_or(crate::Error {
                        missing_field: "signer_name",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: PodCertificateProjectionAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.certificate_chain_path,
            other.certificate_chain_path,
        )?;
        crate::OptionableConvert::merge(
            &mut self.credential_bundle_path,
            other.credential_bundle_path,
        )?;
        crate::OptionableConvert::merge(&mut self.key_path, other.key_path)?;
        if let Some(other_value) = other.key_type {
            crate::OptionableConvert::merge(&mut self.key_type, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.max_expiration_seconds,
            other.max_expiration_seconds,
        )?;
        if let Some(other_value) = other.signer_name {
            crate::OptionableConvert::merge(&mut self.signer_name, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::PodCertificateProjection>
for PodCertificateProjectionAc {
    fn from_optionable(
        value: ::k8s_openapi::api::core::v1::PodCertificateProjection,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::PodCertificateProjection, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::PodCertificateProjection,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
