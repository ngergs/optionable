pub struct PodCertificateRequestStatusAc {
    pub begin_refresh_at: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    pub certificate_chain: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>,
    > as crate::Optionable>::Optioned,
    pub not_after: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    pub not_before: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::certificates::v1alpha1::PodCertificateRequestStatus {
    type Optioned = PodCertificateRequestStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PodCertificateRequestStatusAc {
    type Optioned = PodCertificateRequestStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::certificates::v1alpha1::PodCertificateRequestStatus {
    fn into_optioned(self) -> PodCertificateRequestStatusAc {
        PodCertificateRequestStatusAc {
            begin_refresh_at: crate::OptionableConvert::into_optioned(
                self.begin_refresh_at,
            ),
            certificate_chain: crate::OptionableConvert::into_optioned(
                self.certificate_chain,
            ),
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            not_after: crate::OptionableConvert::into_optioned(self.not_after),
            not_before: crate::OptionableConvert::into_optioned(self.not_before),
        }
    }
    fn try_from_optioned(
        value: PodCertificateRequestStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            begin_refresh_at: crate::OptionableConvert::try_from_optioned(
                value.begin_refresh_at,
            )?,
            certificate_chain: crate::OptionableConvert::try_from_optioned(
                value.certificate_chain,
            )?,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            not_after: crate::OptionableConvert::try_from_optioned(value.not_after)?,
            not_before: crate::OptionableConvert::try_from_optioned(value.not_before)?,
        })
    }
    fn merge(
        &mut self,
        other: PodCertificateRequestStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.begin_refresh_at,
            other.begin_refresh_at,
        )?;
        crate::OptionableConvert::merge(
            &mut self.certificate_chain,
            other.certificate_chain,
        )?;
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        crate::OptionableConvert::merge(&mut self.not_after, other.not_after)?;
        crate::OptionableConvert::merge(&mut self.not_before, other.not_before)?;
        Ok(())
    }
}
