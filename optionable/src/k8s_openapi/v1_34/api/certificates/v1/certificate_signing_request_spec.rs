pub struct CertificateSigningRequestSpecOpt {
    pub expiration_seconds: <Option<i32> as crate::Optionable>::Optioned,
    pub extra: <Option<
        std::collections::BTreeMap<
            std::string::String,
            std::vec::Vec<std::string::String>,
        >,
    > as crate::Optionable>::Optioned,
    pub groups: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub request: Option<<::k8s_openapi::ByteString as crate::Optionable>::Optioned>,
    pub signer_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub uid: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub usages: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub username: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::certificates::v1::CertificateSigningRequestSpec {
    type Optioned = CertificateSigningRequestSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for CertificateSigningRequestSpecOpt {
    type Optioned = CertificateSigningRequestSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::certificates::v1::CertificateSigningRequestSpec {
    fn into_optioned(self) -> CertificateSigningRequestSpecOpt {
        CertificateSigningRequestSpecOpt {
            expiration_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.expiration_seconds),
            extra: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    std::vec::Vec<std::string::String>,
                >,
            > as crate::OptionableConvert>::into_optioned(self.extra),
            groups: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.groups),
            request: Some(
                <::k8s_openapi::ByteString as crate::OptionableConvert>::into_optioned(
                    self.request,
                ),
            ),
            signer_name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.signer_name,
                ),
            ),
            uid: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.uid),
            usages: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.usages),
            username: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.username),
        }
    }
    fn try_from_optioned(
        value: CertificateSigningRequestSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            expiration_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.expiration_seconds)?,
            extra: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    std::vec::Vec<std::string::String>,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.extra)?,
            groups: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.groups)?,
            request: <::k8s_openapi::ByteString as crate::OptionableConvert>::try_from_optioned(
                value
                    .request
                    .ok_or(crate::optionable::Error {
                        missing_field: "request",
                    })?,
            )?,
            signer_name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .signer_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "signer_name",
                    })?,
            )?,
            uid: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.uid)?,
            usages: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.usages)?,
            username: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.username)?,
        })
    }
    fn merge(
        &mut self,
        other: CertificateSigningRequestSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.expiration_seconds,
            other.expiration_seconds,
        )?;
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                std::vec::Vec<std::string::String>,
            >,
        > as crate::OptionableConvert>::merge(&mut self.extra, other.extra)?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.groups, other.groups)?;
        if let Some(other_value) = other.request {
            <::k8s_openapi::ByteString as crate::OptionableConvert>::merge(
                &mut self.request,
                other_value,
            )?;
        }
        if let Some(other_value) = other.signer_name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.signer_name,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.uid, other.uid)?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.usages, other.usages)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.username, other.username)?;
        Ok(())
    }
}
