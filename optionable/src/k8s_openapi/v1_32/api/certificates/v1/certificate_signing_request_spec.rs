#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSigningRequestSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: <Option<
        std::collections::BTreeMap<
            std::string::String,
            std::vec::Vec<std::string::String>,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<<::k8s_openapi::ByteString as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usages: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::certificates::v1::CertificateSigningRequestSpec {
    type Optioned = CertificateSigningRequestSpecAc;
}
#[automatically_derived]
impl crate::Optionable for CertificateSigningRequestSpecAc {
    type Optioned = CertificateSigningRequestSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::certificates::v1::CertificateSigningRequestSpec {
    fn into_optioned(self) -> CertificateSigningRequestSpecAc {
        CertificateSigningRequestSpecAc {
            expiration_seconds: crate::OptionableConvert::into_optioned(
                self.expiration_seconds,
            ),
            extra: crate::OptionableConvert::into_optioned(self.extra),
            groups: crate::OptionableConvert::into_optioned(self.groups),
            request: Some(crate::OptionableConvert::into_optioned(self.request)),
            signer_name: Some(crate::OptionableConvert::into_optioned(self.signer_name)),
            uid: crate::OptionableConvert::into_optioned(self.uid),
            usages: crate::OptionableConvert::into_optioned(self.usages),
            username: crate::OptionableConvert::into_optioned(self.username),
        }
    }
    fn try_from_optioned(
        value: CertificateSigningRequestSpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            expiration_seconds: crate::OptionableConvert::try_from_optioned(
                value.expiration_seconds,
            )?,
            extra: crate::OptionableConvert::try_from_optioned(value.extra)?,
            groups: crate::OptionableConvert::try_from_optioned(value.groups)?,
            request: crate::OptionableConvert::try_from_optioned(
                value
                    .request
                    .ok_or(crate::optionable::Error {
                        missing_field: "request",
                    })?,
            )?,
            signer_name: crate::OptionableConvert::try_from_optioned(
                value
                    .signer_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "signer_name",
                    })?,
            )?,
            uid: crate::OptionableConvert::try_from_optioned(value.uid)?,
            usages: crate::OptionableConvert::try_from_optioned(value.usages)?,
            username: crate::OptionableConvert::try_from_optioned(value.username)?,
        })
    }
    fn merge(
        &mut self,
        other: CertificateSigningRequestSpecAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.expiration_seconds,
            other.expiration_seconds,
        )?;
        crate::OptionableConvert::merge(&mut self.extra, other.extra)?;
        crate::OptionableConvert::merge(&mut self.groups, other.groups)?;
        if let Some(other_value) = other.request {
            crate::OptionableConvert::merge(&mut self.request, other_value)?;
        }
        if let Some(other_value) = other.signer_name {
            crate::OptionableConvert::merge(&mut self.signer_name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.uid, other.uid)?;
        crate::OptionableConvert::merge(&mut self.usages, other.usages)?;
        crate::OptionableConvert::merge(&mut self.username, other.username)?;
        Ok(())
    }
}
