pub struct PodCertificateProjectionAc {
    pub certificate_chain_path: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub credential_bundle_path: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub key_path: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub key_type: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub max_expiration_seconds: <Option<i32> as crate::Optionable>::Optioned,
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
    ) -> Result<Self, crate::optionable::Error> {
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
                    .ok_or(crate::optionable::Error {
                        missing_field: "key_type",
                    })?,
            )?,
            max_expiration_seconds: crate::OptionableConvert::try_from_optioned(
                value.max_expiration_seconds,
            )?,
            signer_name: crate::OptionableConvert::try_from_optioned(
                value
                    .signer_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "signer_name",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PodCertificateProjectionAc,
    ) -> Result<(), crate::optionable::Error> {
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
