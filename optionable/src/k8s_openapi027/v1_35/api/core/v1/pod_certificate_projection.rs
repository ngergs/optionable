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
    pub certificate_chain_path: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_bundle_path: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_path: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_expiration_seconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_annotations: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PodCertificateProjection {
    type Optioned = PodCertificateProjectionAc;
}
#[automatically_derived]
impl crate::Optionable for PodCertificateProjectionAc {
    type Optioned = PodCertificateProjectionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::PodCertificateProjection {
    fn into_optioned(self) -> PodCertificateProjectionAc {
        PodCertificateProjectionAc {
            certificate_chain_path: self.certificate_chain_path,
            credential_bundle_path: self.credential_bundle_path,
            key_path: self.key_path,
            key_type: Some(self.key_type),
            max_expiration_seconds: self.max_expiration_seconds,
            signer_name: Some(self.signer_name),
            user_annotations: self.user_annotations,
        }
    }
    fn try_from_optioned(
        value: PodCertificateProjectionAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            certificate_chain_path: value.certificate_chain_path,
            credential_bundle_path: value.credential_bundle_path,
            key_path: value.key_path,
            key_type: value
                .key_type
                .ok_or(crate::Error {
                    missing_field: "key_type",
                })?,
            max_expiration_seconds: value.max_expiration_seconds,
            signer_name: value
                .signer_name
                .ok_or(crate::Error {
                    missing_field: "signer_name",
                })?,
            user_annotations: value.user_annotations,
        })
    }
    fn merge(&mut self, other: PodCertificateProjectionAc) -> Result<(), crate::Error> {
        self.certificate_chain_path = other.certificate_chain_path;
        self.credential_bundle_path = other.credential_bundle_path;
        self.key_path = other.key_path;
        if let Some(other_value) = other.key_type {
            self.key_type = other_value;
        }
        self.max_expiration_seconds = other.max_expiration_seconds;
        if let Some(other_value) = other.signer_name {
            self.signer_name = other_value;
        }
        self.user_annotations = other.user_annotations;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PodCertificateProjection>
for PodCertificateProjectionAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::PodCertificateProjection,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PodCertificateProjection, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PodCertificateProjection,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
