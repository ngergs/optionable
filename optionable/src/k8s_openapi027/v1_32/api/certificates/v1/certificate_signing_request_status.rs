#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CertificateSigningRequestStatus contains conditions used to indicate approved/denied/failed status of the request, and the issued certificate.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CertificateSigningRequestStatusAc {
    /// certificate is populated with an issued certificate by the signer after an Approved condition is present. This field is set via the /status subresource. Once populated, this field is immutable.
    ///
    /// If the certificate signing request is denied, a condition of type "Denied" is added and this field remains empty. If the signer cannot issue the certificate, a condition of type "Failed" is added and this field remains empty.
    ///
    /// Validation requirements:
    ///  1. certificate must contain one or more PEM blocks.
    ///  2. All PEM blocks must have the "CERTIFICATE" label, contain no headers, and the encoded data
    ///   must be a BER-encoded ASN.1 Certificate structure as described in section 4 of RFC5280.
    ///  3. Non-PEM content may appear before or after the "CERTIFICATE" PEM blocks and is unvalidated,
    ///   to allow for explanatory text as described in section 5.2 of RFC7468.
    ///
    /// If more than one PEM block is present, and the definition of the requested spec.signerName does not indicate otherwise, the first block is the issued certificate, and subsequent blocks should be treated as intermediate certificates and presented in TLS handshakes.
    ///
    /// The certificate is encoded in PEM format.
    ///
    /// When serialized as JSON or YAML, the data is additionally base64-encoded, so it consists of:
    ///
    ///   base64(
    ///     -----BEGIN CERTIFICATE-----
    ///     ...
    ///     -----END CERTIFICATE-----
    ///     )
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<
        <::k8s_openapi027::ByteString as crate::Optionable>::Optioned,
    >,
    /// conditions applied to the request. Known conditions are "Approved", "Denied", and "Failed".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::certificates::v1::CertificateSigningRequestCondition as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::certificates::v1::CertificateSigningRequestStatus {
    type Optioned = CertificateSigningRequestStatusAc;
}
#[automatically_derived]
impl crate::Optionable for CertificateSigningRequestStatusAc {
    type Optioned = CertificateSigningRequestStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::certificates::v1::CertificateSigningRequestStatus {
    fn into_optioned(self) -> CertificateSigningRequestStatusAc {
        CertificateSigningRequestStatusAc {
            certificate: crate::OptionableConvert::into_optioned(self.certificate),
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
        }
    }
    fn try_from_optioned(
        value: CertificateSigningRequestStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            certificate: crate::OptionableConvert::try_from_optioned(value.certificate)?,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
        })
    }
    fn merge(
        &mut self,
        other: CertificateSigningRequestStatusAc,
    ) -> Result<(), crate::Error> {
        if self.certificate.is_none() {
            self.certificate = other.certificate;
        }
        if let Some(other_value) = other.certificate {
            crate::OptionableConvert::merge(&mut self.certificate, other_value)?;
        }
        if self.conditions.is_none() {
            self.conditions = other.conditions;
        }
        if let Some(other_value) = other.conditions {
            crate::merge::try_merge_optioned_map(&mut self.conditions, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::certificates::v1::CertificateSigningRequestStatus,
> for CertificateSigningRequestStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::certificates::v1::CertificateSigningRequestStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::certificates::v1::CertificateSigningRequestStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::certificates::v1::CertificateSigningRequestStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
