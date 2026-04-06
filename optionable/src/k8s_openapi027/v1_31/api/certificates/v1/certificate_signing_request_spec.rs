#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CertificateSigningRequestSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<
        std::collections::BTreeMap<
            std::string::String,
            std::vec::Vec<std::string::String>,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<<::k8s_openapi027::ByteString as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usages: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::certificates::v1::CertificateSigningRequestSpec {
    type Optioned = CertificateSigningRequestSpecAc;
}
#[automatically_derived]
impl crate::Optionable for CertificateSigningRequestSpecAc {
    type Optioned = CertificateSigningRequestSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::certificates::v1::CertificateSigningRequestSpec {
    fn into_optioned(self) -> CertificateSigningRequestSpecAc {
        CertificateSigningRequestSpecAc {
            expiration_seconds: self.expiration_seconds,
            extra: self.extra,
            groups: self.groups,
            request: Some(crate::OptionableConvert::into_optioned(self.request)),
            signer_name: Some(self.signer_name),
            uid: self.uid,
            usages: self.usages,
            username: self.username,
        }
    }
    fn try_from_optioned(
        value: CertificateSigningRequestSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            expiration_seconds: value.expiration_seconds,
            extra: value.extra,
            groups: value.groups,
            request: crate::OptionableConvert::try_from_optioned(
                value
                    .request
                    .ok_or(crate::Error {
                        missing_field: "request",
                    })?,
            )?,
            signer_name: value
                .signer_name
                .ok_or(crate::Error {
                    missing_field: "signer_name",
                })?,
            uid: value.uid,
            usages: value.usages,
            username: value.username,
        })
    }
    fn merge(
        &mut self,
        other: CertificateSigningRequestSpecAc,
    ) -> Result<(), crate::Error> {
        self.expiration_seconds = other.expiration_seconds;
        self.extra = other.extra;
        self.groups = other.groups;
        if let Some(other_value) = other.request {
            crate::OptionableConvert::merge(&mut self.request, other_value)?;
        }
        if let Some(other_value) = other.signer_name {
            self.signer_name = other_value;
        }
        self.uid = other.uid;
        self.usages = other.usages;
        self.username = other.username;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::certificates::v1::CertificateSigningRequestSpec,
> for CertificateSigningRequestSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::certificates::v1::CertificateSigningRequestSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::certificates::v1::CertificateSigningRequestSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::certificates::v1::CertificateSigningRequestSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
