#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodCertificateProjection provides a private key and X.509 certificate in the pod filesystem.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodCertificateProjectionAc {
    /// Write the certificate chain at this path in the projected volume.
    ///
    /// Most applications should use credentialBundlePath.  When using keyPath and certificateChainPath, your application needs to check that the key and leaf certificate are consistent, because it is possible to read the files mid-rotation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain_path: Option<std::string::String>,
    /// Write the credential bundle at this path in the projected volume.
    ///
    /// The credential bundle is a single file that contains multiple PEM blocks. The first PEM block is a PRIVATE KEY block, containing a PKCS#8 private key.
    ///
    /// The remaining blocks are CERTIFICATE blocks, containing the issued certificate chain from the signer (leaf and any intermediates).
    ///
    /// Using credentialBundlePath lets your Pod's application code make a single atomic read that retrieves a consistent key and certificate chain.  If you project them to separate files, your application code will need to additionally check that the leaf certificate was issued to the key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_bundle_path: Option<std::string::String>,
    /// Write the key at this path in the projected volume.
    ///
    /// Most applications should use credentialBundlePath.  When using keyPath and certificateChainPath, your application needs to check that the key and leaf certificate are consistent, because it is possible to read the files mid-rotation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_path: Option<std::string::String>,
    /// The type of keypair Kubelet will generate for the pod.
    ///
    /// Valid values are "RSA3072", "RSA4096", "ECDSAP256", "ECDSAP384", "ECDSAP521", and "ED25519".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<std::string::String>,
    /// maxExpirationSeconds is the maximum lifetime permitted for the certificate.
    ///
    /// Kubelet copies this value verbatim into the PodCertificateRequests it generates for this projection.
    ///
    /// If omitted, kube-apiserver will set it to 86400(24 hours). kube-apiserver will reject values shorter than 3600 (1 hour).  The maximum allowable value is 7862400 (91 days).
    ///
    /// The signer implementation is then free to issue a certificate with any lifetime *shorter* than MaxExpirationSeconds, but no shorter than 3600 seconds (1 hour).  This constraint is enforced by kube-apiserver. `kubernetes.io` signers will never issue certificates with a lifetime longer than 24 hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_expiration_seconds: Option<i32>,
    /// Kubelet's generated CSRs will be addressed to this signer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_name: Option<std::string::String>,
    /// userAnnotations allow pod authors to pass additional information to the signer implementation.  Kubernetes does not restrict or validate this metadata in any way.
    ///
    /// These values are copied verbatim into the `spec.unverifiedUserAnnotations` field of the PodCertificateRequest objects that Kubelet creates.
    ///
    /// Entries are subject to the same validation as object metadata annotations, with the addition that all keys must be domain-prefixed. No restrictions are placed on values, except an overall size limitation on the entire field.
    ///
    /// Signers should document the keys and values they support. Signers should deny requests that contain keys they do not recognize.
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
        if self.certificate_chain_path.is_none() {
            self.certificate_chain_path = crate::OptionableConvert::try_from_optioned(
                other.certificate_chain_path,
            )?;
        } else if let Some(self_value) = self.certificate_chain_path.as_mut()
            && let Some(other_value) = other.certificate_chain_path
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.credential_bundle_path.is_none() {
            self.credential_bundle_path = crate::OptionableConvert::try_from_optioned(
                other.credential_bundle_path,
            )?;
        } else if let Some(self_value) = self.credential_bundle_path.as_mut()
            && let Some(other_value) = other.credential_bundle_path
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.key_path.is_none() {
            self.key_path = crate::OptionableConvert::try_from_optioned(other.key_path)?;
        } else if let Some(self_value) = self.key_path.as_mut()
            && let Some(other_value) = other.key_path
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.key_type {
            self.key_type = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.max_expiration_seconds.is_none() {
            self.max_expiration_seconds = crate::OptionableConvert::try_from_optioned(
                other.max_expiration_seconds,
            )?;
        } else if let Some(self_value) = self.max_expiration_seconds.as_mut()
            && let Some(other_value) = other.max_expiration_seconds
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.signer_name {
            self.signer_name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.user_annotations.is_none() {
            self.user_annotations = crate::OptionableConvert::try_from_optioned(
                other.user_annotations,
            )?;
        } else if let Some(self_value) = self.user_annotations.as_mut()
            && let Some(other_value) = other.user_annotations
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
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
impl k8s_openapi027::DeepMerge for PodCertificateProjectionAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.certificate_chain_path,
            other.certificate_chain_path,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.credential_bundle_path,
            other.credential_bundle_path,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.key_path, other.key_path);
        k8s_openapi027::DeepMerge::merge_from(&mut self.key_type, other.key_type);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.max_expiration_seconds,
            other.max_expiration_seconds,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.signer_name, other.signer_name);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.user_annotations,
            other.user_annotations,
        );
    }
}
