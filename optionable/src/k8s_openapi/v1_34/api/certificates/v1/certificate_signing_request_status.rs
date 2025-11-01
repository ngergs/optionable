pub struct CertificateSigningRequestStatusAc {
    pub certificate: <Option<::k8s_openapi::ByteString> as crate::Optionable>::Optioned,
    pub conditions: <Option<
        std::vec::Vec<
            ::k8s_openapi::api::certificates::v1::CertificateSigningRequestCondition,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::certificates::v1::CertificateSigningRequestStatus {
    type Optioned = CertificateSigningRequestStatusAc;
}
#[automatically_derived]
impl crate::Optionable for CertificateSigningRequestStatusAc {
    type Optioned = CertificateSigningRequestStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::certificates::v1::CertificateSigningRequestStatus {
    fn into_optioned(self) -> CertificateSigningRequestStatusAc {
        CertificateSigningRequestStatusAc {
            certificate: crate::OptionableConvert::into_optioned(self.certificate),
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
        }
    }
    fn try_from_optioned(
        value: CertificateSigningRequestStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            certificate: crate::OptionableConvert::try_from_optioned(value.certificate)?,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
        })
    }
    fn merge(
        &mut self,
        other: CertificateSigningRequestStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.certificate, other.certificate)?;
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        Ok(())
    }
}
