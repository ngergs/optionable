#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodCertificateRequestStatus describes the status of the request, and holds the certificate data if the request is issued.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodCertificateRequestStatusAc {
    /// beginRefreshAt is the time at which the kubelet should begin trying to refresh the certificate.  This field is set via the /status subresource, and must be set at the same time as certificateChain.  Once populated, this field is immutable.
    ///
    /// This field is only a hint.  Kubelet may start refreshing before or after this time if necessary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_refresh_at: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// certificateChain is populated with an issued certificate by the signer. This field is set via the /status subresource. Once populated, this field is immutable.
    ///
    /// If the certificate signing request is denied, a condition of type "Denied" is added and this field remains empty. If the signer cannot issue the certificate, a condition of type "Failed" is added and this field remains empty.
    ///
    /// Validation requirements:
    ///  1. certificateChain must consist of one or more PEM-formatted certificates.
    ///  2. Each entry must be a valid PEM-wrapped, DER-encoded ASN.1 Certificate as
    ///     described in section 4 of RFC5280.
    ///
    /// If more than one block is present, and the definition of the requested spec.signerName does not indicate otherwise, the first block is the issued certificate, and subsequent blocks should be treated as intermediate certificates and presented in TLS handshakes.  When projecting the chain into a pod volume, kubelet will drop any data in-between the PEM blocks, as well as any PEM block headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<std::string::String>,
    /// conditions applied to the request.
    ///
    /// The types "Issued", "Denied", and "Failed" have special handling.  At most one of these conditions may be present, and they must have status "True".
    ///
    /// If the request is denied with `Reason=UnsupportedKeyType`, the signer may suggest a key type that will work in the message field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Condition as crate::Optionable>::Optioned,
        >,
    >,
    /// notAfter is the time at which the certificate expires.  The value must be the same as the notAfter value in the leaf certificate in certificateChain.  This field is set via the /status subresource.  Once populated, it is immutable.  The signer must set this field at the same time it sets certificateChain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// notBefore is the time at which the certificate becomes valid.  The value must be the same as the notBefore value in the leaf certificate in certificateChain.  This field is set via the /status subresource.  Once populated, it is immutable. The signer must set this field at the same time it sets certificateChain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequestStatus {
    type Optioned = PodCertificateRequestStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PodCertificateRequestStatusAc {
    type Optioned = PodCertificateRequestStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequestStatus {
    fn into_optioned(self) -> PodCertificateRequestStatusAc {
        PodCertificateRequestStatusAc {
            begin_refresh_at: crate::OptionableConvert::into_optioned(
                self.begin_refresh_at,
            ),
            certificate_chain: self.certificate_chain,
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            not_after: crate::OptionableConvert::into_optioned(self.not_after),
            not_before: crate::OptionableConvert::into_optioned(self.not_before),
        }
    }
    fn try_from_optioned(
        value: PodCertificateRequestStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            begin_refresh_at: crate::OptionableConvert::try_from_optioned(
                value.begin_refresh_at,
            )?,
            certificate_chain: value.certificate_chain,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            not_after: crate::OptionableConvert::try_from_optioned(value.not_after)?,
            not_before: crate::OptionableConvert::try_from_optioned(value.not_before)?,
        })
    }
    fn merge(
        &mut self,
        other: PodCertificateRequestStatusAc,
    ) -> Result<(), crate::Error> {
        if self.begin_refresh_at.is_none() {
            self.begin_refresh_at = crate::OptionableConvert::try_from_optioned(
                other.begin_refresh_at,
            )?;
        } else if let Some(self_value) = self.begin_refresh_at.as_mut()
            && let Some(other_value) = other.begin_refresh_at
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.certificate_chain.is_none() {
            self.certificate_chain = crate::OptionableConvert::try_from_optioned(
                other.certificate_chain,
            )?;
        } else if let Some(self_value) = self.certificate_chain.as_mut()
            && let Some(other_value) = other.certificate_chain
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.conditions.is_none() {
            self.conditions = crate::OptionableConvert::try_from_optioned(
                other.conditions,
            )?;
        } else if let Some(self_value) = self.conditions.as_mut()
            && let Some(other_value) = other.conditions
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.not_after.is_none() {
            self.not_after = crate::OptionableConvert::try_from_optioned(
                other.not_after,
            )?;
        } else if let Some(self_value) = self.not_after.as_mut()
            && let Some(other_value) = other.not_after
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.not_before.is_none() {
            self.not_before = crate::OptionableConvert::try_from_optioned(
                other.not_before,
            )?;
        } else if let Some(self_value) = self.not_before.as_mut()
            && let Some(other_value) = other.not_before
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequestStatus,
> for PodCertificateRequestStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequestStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequestStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::certificates::v1alpha1::PodCertificateRequestStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for PodCertificateRequestStatusAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.begin_refresh_at,
            other.begin_refresh_at,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.certificate_chain,
            other.certificate_chain,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.conditions, other.conditions);
        k8s_openapi027::DeepMerge::merge_from(&mut self.not_after, other.not_after);
        k8s_openapi027::DeepMerge::merge_from(&mut self.not_before, other.not_before);
    }
}
