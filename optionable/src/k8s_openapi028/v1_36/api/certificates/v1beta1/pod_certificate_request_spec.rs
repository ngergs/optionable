#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodCertificateRequestSpec describes the certificate request.  All fields are immutable after creation.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodCertificateRequestSpecAc {
    /// maxExpirationSeconds is the maximum lifetime permitted for the certificate.
    ///
    /// If omitted, kube-apiserver will set it to 86400(24 hours). kube-apiserver will reject values shorter than 3600 (1 hour).  The maximum allowable value is 7862400 (91 days).
    ///
    /// The signer implementation is then free to issue a certificate with any lifetime *shorter* than MaxExpirationSeconds, but no shorter than 3600 seconds (1 hour).  This constraint is enforced by kube-apiserver. `kubernetes.io` signers will never issue certificates with a lifetime longer than 24 hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_expiration_seconds: Option<i32>,
    /// nodeName is the name of the node the pod is assigned to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<std::string::String>,
    /// nodeUID is the UID of the node the pod is assigned to.
    #[serde(rename = "nodeUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_uid: Option<std::string::String>,
    /// The PKIX-serialized public key the signer will issue the certificate to.
    ///
    /// The key must be one of RSA3072, RSA4096, ECDSAP256, ECDSAP384, ECDSAP521, or ED25519. Note that this list may be expanded in the future.
    ///
    /// Signer implementations do not need to support all key types supported by kube-apiserver and kubelet.  If a signer does not support the key type used for a given PodCertificateRequest, it must deny the request by setting a status.conditions entry with a type of "Denied" and a reason of "UnsupportedKeyType". It may also suggest a key type that it does support in the message field.
    ///
    /// Deprecated: This field is replaced by StubPKCS10Request. If StubPKCS10Request is set, this field must be empty.  Signer implementations should extract the public key from the StubPKCS10Request field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkix_public_key: Option<
        <::k8s_openapi028::ByteString as crate::Optionable>::Optioned,
    >,
    /// podName is the name of the pod into which the certificate will be mounted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_name: Option<std::string::String>,
    /// podUID is the UID of the pod into which the certificate will be mounted.
    #[serde(rename = "podUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_uid: Option<std::string::String>,
    /// A proof that the requesting kubelet holds the private key corresponding to pkixPublicKey.
    ///
    /// It is contructed by signing the ASCII bytes of the pod's UID using `pkixPublicKey`.
    ///
    /// kube-apiserver validates the proof of possession during creation of the PodCertificateRequest.
    ///
    /// If the key is an RSA key, then the signature is over the ASCII bytes of the pod UID, using RSASSA-PSS from RFC 8017 (as implemented by the golang function crypto/rsa.SignPSS with nil options).
    ///
    /// If the key is an ECDSA key, then the signature is as described by \[SEC 1, Version 2.0\](https://www.secg.org/sec1-v2.pdf) (as implemented by the golang library function crypto/ecdsa.SignASN1)
    ///
    /// If the key is an ED25519 key, the the signature is as described by the \[ED25519 Specification\](https://ed25519.cr.yp.to/) (as implemented by the golang library crypto/ed25519.Sign).
    ///
    /// Deprecated: This field is replaced by StubPKCS10Request. If StubPKCS10Request is set, this field must be empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_of_possession: Option<
        <::k8s_openapi028::ByteString as crate::Optionable>::Optioned,
    >,
    /// serviceAccountName is the name of the service account the pod is running as.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_name: Option<std::string::String>,
    /// serviceAccountUID is the UID of the service account the pod is running as.
    #[serde(rename = "serviceAccountUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_uid: Option<std::string::String>,
    /// signerName indicates the requested signer.
    ///
    /// All signer names beginning with `kubernetes.io` are reserved for use by the Kubernetes project.  There is currently one well-known signer documented by the Kubernetes project, `kubernetes.io/kube-apiserver-client-pod`, which will issue client certificates understood by kube-apiserver.  It is currently unimplemented.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_name: Option<std::string::String>,
    /// A PKCS#10 certificate signing request (DER-serialized) generated by Kubelet using the subject private key.
    ///
    /// Most signer implementations will ignore the contents of the CSR except to extract the subject public key. The API server automatically verifies the CSR signature during admission, so the signer does not need to repeat the verification.  CSRs generated by kubelet are completely empty.
    ///
    /// The subject public key must be one of RSA3072, RSA4096, ECDSAP256, ECDSAP384, ECDSAP521, or ED25519. Note that this list may be expanded in the future.
    ///
    /// Signer implementations do not need to support all key types supported by kube-apiserver and kubelet.  If a signer does not support the key type used for a given PodCertificateRequest, it must deny the request by setting a status.conditions entry with a type of "Denied" and a reason of "UnsupportedKeyType". It may also suggest a key type that it does support in the message field.
    #[serde(rename = "stubPKCS10Request")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stub_pkc_s10_request: Option<
        <::k8s_openapi028::ByteString as crate::Optionable>::Optioned,
    >,
    /// unverifiedUserAnnotations allow pod authors to pass additional information to the signer implementation.  Kubernetes does not restrict or validate this metadata in any way.
    ///
    /// Entries are subject to the same validation as object metadata annotations, with the addition that all keys must be domain-prefixed. No restrictions are placed on values, except an overall size limitation on the entire field.
    ///
    /// Signers should document the keys and values they support.  Signers should deny requests that contain keys they do not recognize.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unverified_user_annotations: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi028::api::certificates::v1beta1::PodCertificateRequestSpec {
    type Optioned = PodCertificateRequestSpecAc;
}
#[automatically_derived]
impl crate::Optionable for PodCertificateRequestSpecAc {
    type Optioned = PodCertificateRequestSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi028::api::certificates::v1beta1::PodCertificateRequestSpec {
    fn into_optioned(self) -> PodCertificateRequestSpecAc {
        PodCertificateRequestSpecAc {
            max_expiration_seconds: self.max_expiration_seconds,
            node_name: Some(self.node_name),
            node_uid: Some(self.node_uid),
            pkix_public_key: crate::OptionableConvert::into_optioned(
                self.pkix_public_key,
            ),
            pod_name: Some(self.pod_name),
            pod_uid: Some(self.pod_uid),
            proof_of_possession: crate::OptionableConvert::into_optioned(
                self.proof_of_possession,
            ),
            service_account_name: Some(self.service_account_name),
            service_account_uid: Some(self.service_account_uid),
            signer_name: Some(self.signer_name),
            stub_pkc_s10_request: Some(
                crate::OptionableConvert::into_optioned(self.stub_pkc_s10_request),
            ),
            unverified_user_annotations: self.unverified_user_annotations,
        }
    }
    fn try_from_optioned(
        value: PodCertificateRequestSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            max_expiration_seconds: value.max_expiration_seconds,
            node_name: value
                .node_name
                .ok_or(crate::Error {
                    missing_field: "node_name",
                })?,
            node_uid: value
                .node_uid
                .ok_or(crate::Error {
                    missing_field: "node_uid",
                })?,
            pkix_public_key: crate::OptionableConvert::try_from_optioned(
                value.pkix_public_key,
            )?,
            pod_name: value
                .pod_name
                .ok_or(crate::Error {
                    missing_field: "pod_name",
                })?,
            pod_uid: value
                .pod_uid
                .ok_or(crate::Error {
                    missing_field: "pod_uid",
                })?,
            proof_of_possession: crate::OptionableConvert::try_from_optioned(
                value.proof_of_possession,
            )?,
            service_account_name: value
                .service_account_name
                .ok_or(crate::Error {
                    missing_field: "service_account_name",
                })?,
            service_account_uid: value
                .service_account_uid
                .ok_or(crate::Error {
                    missing_field: "service_account_uid",
                })?,
            signer_name: value
                .signer_name
                .ok_or(crate::Error {
                    missing_field: "signer_name",
                })?,
            stub_pkc_s10_request: crate::OptionableConvert::try_from_optioned(
                value
                    .stub_pkc_s10_request
                    .ok_or(crate::Error {
                        missing_field: "stub_pkc_s10_request",
                    })?,
            )?,
            unverified_user_annotations: value.unverified_user_annotations,
        })
    }
    fn merge(&mut self, other: PodCertificateRequestSpecAc) -> Result<(), crate::Error> {
        if self.max_expiration_seconds.is_none() {
            self.max_expiration_seconds = crate::OptionableConvert::try_from_optioned(
                other.max_expiration_seconds,
            )?;
        } else if let Some(self_value) = self.max_expiration_seconds.as_mut()
            && let Some(other_value) = other.max_expiration_seconds
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.node_name {
            self.node_name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.node_uid {
            self.node_uid = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.pkix_public_key.is_none() {
            self.pkix_public_key = crate::OptionableConvert::try_from_optioned(
                other.pkix_public_key,
            )?;
        } else if let Some(self_value) = self.pkix_public_key.as_mut()
            && let Some(other_value) = other.pkix_public_key
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.pod_name {
            self.pod_name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.pod_uid {
            self.pod_uid = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.proof_of_possession.is_none() {
            self.proof_of_possession = crate::OptionableConvert::try_from_optioned(
                other.proof_of_possession,
            )?;
        } else if let Some(self_value) = self.proof_of_possession.as_mut()
            && let Some(other_value) = other.proof_of_possession
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.service_account_name {
            self.service_account_name = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if let Some(other_value) = other.service_account_uid {
            self.service_account_uid = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if let Some(other_value) = other.signer_name {
            self.signer_name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.stub_pkc_s10_request {
            crate::OptionableConvert::merge(
                &mut self.stub_pkc_s10_request,
                other_value,
            )?;
        }
        if self.unverified_user_annotations.is_none() {
            self.unverified_user_annotations = crate::OptionableConvert::try_from_optioned(
                other.unverified_user_annotations,
            )?;
        } else if let Some(self_value) = self.unverified_user_annotations.as_mut()
            && let Some(other_value) = other.unverified_user_annotations
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi028::api::certificates::v1beta1::PodCertificateRequestSpec,
> for PodCertificateRequestSpecAc {
    fn from_optionable(
        value: k8s_openapi028::api::certificates::v1beta1::PodCertificateRequestSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi028::api::certificates::v1beta1::PodCertificateRequestSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::api::certificates::v1beta1::PodCertificateRequestSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::DeepMerge for PodCertificateRequestSpecAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.max_expiration_seconds,
            other.max_expiration_seconds,
        );
        k8s_openapi028::DeepMerge::merge_from(&mut self.node_name, other.node_name);
        k8s_openapi028::DeepMerge::merge_from(&mut self.node_uid, other.node_uid);
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.pkix_public_key,
            other.pkix_public_key,
        );
        k8s_openapi028::DeepMerge::merge_from(&mut self.pod_name, other.pod_name);
        k8s_openapi028::DeepMerge::merge_from(&mut self.pod_uid, other.pod_uid);
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.proof_of_possession,
            other.proof_of_possession,
        );
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.service_account_name,
            other.service_account_name,
        );
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.service_account_uid,
            other.service_account_uid,
        );
        k8s_openapi028::DeepMerge::merge_from(&mut self.signer_name, other.signer_name);
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.stub_pkc_s10_request,
            other.stub_pkc_s10_request,
        );
        crate::k8s_openapi::merge::merge_granular_option_wrapped(
            &mut self.unverified_user_annotations,
            other.unverified_user_annotations,
        );
    }
}
