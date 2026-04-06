#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CertificateSigningRequestSpec contains the certificate request.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CertificateSigningRequestSpecAc {
    /// expirationSeconds is the requested duration of validity of the issued certificate. The certificate signer may issue a certificate with a different validity duration so a client must check the delta between the notBefore and and notAfter fields in the issued certificate to determine the actual duration.
    ///
    /// The v1.22+ in-tree implementations of the well-known Kubernetes signers will honor this field as long as the requested duration is not greater than the maximum duration they will honor per the --cluster-signing-duration CLI flag to the Kubernetes controller manager.
    ///
    /// Certificate signers may not honor this field for various reasons:
    ///
    ///   1. Old signer that is unaware of the field (such as the in-tree
    ///      implementations prior to v1.22)
    ///   2. Signer whose configured maximum is shorter than the requested duration
    ///   3. Signer whose configured minimum is longer than the requested duration
    ///
    /// The minimum valid value for expirationSeconds is 600, i.e. 10 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: Option<i32>,
    /// extra contains extra attributes of the user that created the CertificateSigningRequest. Populated by the API server on creation and immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<
        std::collections::BTreeMap<
            std::string::String,
            std::vec::Vec<std::string::String>,
        >,
    >,
    /// groups contains group membership of the user that created the CertificateSigningRequest. Populated by the API server on creation and immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<std::vec::Vec<std::string::String>>,
    /// request contains an x509 certificate signing request encoded in a "CERTIFICATE REQUEST" PEM block. When serialized as JSON or YAML, the data is additionally base64-encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<<::k8s_openapi027::ByteString as crate::Optionable>::Optioned>,
    /// signerName indicates the requested signer, and is a qualified name.
    ///
    /// List/watch requests for CertificateSigningRequests can filter on this field using a "spec.signerName=NAME" fieldSelector.
    ///
    /// Well-known Kubernetes signers are:
    ///  1. "kubernetes.io/kube-apiserver-client": issues client certificates that can be used to authenticate to kube-apiserver.
    ///   Requests for this signer are never auto-approved by kube-controller-manager, can be issued by the "csrsigning" controller in kube-controller-manager.
    ///  2. "kubernetes.io/kube-apiserver-client-kubelet": issues client certificates that kubelets use to authenticate to kube-apiserver.
    ///   Requests for this signer can be auto-approved by the "csrapproving" controller in kube-controller-manager, and can be issued by the "csrsigning" controller in kube-controller-manager.
    ///  3. "kubernetes.io/kubelet-serving" issues serving certificates that kubelets use to serve TLS endpoints, which kube-apiserver can connect to securely.
    ///   Requests for this signer are never auto-approved by kube-controller-manager, and can be issued by the "csrsigning" controller in kube-controller-manager.
    ///
    /// More details are available at https://k8s.io/docs/reference/access-authn-authz/certificate-signing-requests/#kubernetes-signers
    ///
    /// Custom signerNames can also be specified. The signer defines:
    ///  1. Trust distribution: how trust (CA bundles) are distributed.
    ///  2. Permitted subjects: and behavior when a disallowed subject is requested.
    ///  3. Required, permitted, or forbidden x509 extensions in the request (including whether subjectAltNames are allowed, which types, restrictions on allowed values) and behavior when a disallowed extension is requested.
    ///  4. Required, permitted, or forbidden key usages / extended key usages.
    ///  5. Expiration/certificate lifetime: whether it is fixed by the signer, configurable by the admin.
    ///  6. Whether or not requests for CA certificates are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_name: Option<std::string::String>,
    /// uid contains the uid of the user that created the CertificateSigningRequest. Populated by the API server on creation and immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<std::string::String>,
    /// usages specifies a set of key usages requested in the issued certificate.
    ///
    /// Requests for TLS client certificates typically request: "digital signature", "key encipherment", "client auth".
    ///
    /// Requests for TLS serving certificates typically request: "key encipherment", "digital signature", "server auth".
    ///
    /// Valid values are:
    ///  "signing", "digital signature", "content commitment",
    ///  "key encipherment", "key agreement", "data encipherment",
    ///  "cert sign", "crl sign", "encipher only", "decipher only", "any",
    ///  "server auth", "client auth",
    ///  "code signing", "email protection", "s/mime",
    ///  "ipsec end system", "ipsec tunnel", "ipsec user",
    ///  "timestamping", "ocsp signing", "microsoft sgc", "netscape sgc"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usages: Option<std::vec::Vec<std::string::String>>,
    /// username contains the name of the user that created the CertificateSigningRequest. Populated by the API server on creation and immutable.
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
