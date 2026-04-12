#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ClusterTrustBundleSpec contains the signer and trust anchors.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClusterTrustBundleSpecAc {
    /// signerName indicates the associated signer, if any.
    ///
    /// In order to create or update a ClusterTrustBundle that sets signerName, you must have the following cluster-scoped permission: group=certificates.k8s.io resource=signers resourceName=\<the signer name\> verb=attest.
    ///
    /// If signerName is not empty, then the ClusterTrustBundle object must be named with the signer name as a prefix (translating slashes to colons). For example, for the signer name `example.com/foo`, valid ClusterTrustBundle object names include `example.com:foo:abc` and `example.com:foo:v1`.
    ///
    /// If signerName is empty, then the ClusterTrustBundle object's name must not have such a prefix.
    ///
    /// List/watch requests for ClusterTrustBundles can filter on this field using a `spec.signerName=NAME` field selector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_name: Option<std::string::String>,
    /// trustBundle contains the individual X.509 trust anchors for this bundle, as PEM bundle of PEM-wrapped, DER-formatted X.509 certificates.
    ///
    /// The data must consist only of PEM certificate blocks that parse as valid X.509 certificates.  Each certificate must include a basic constraints extension with the CA bit set.  The API server will reject objects that contain duplicate certificates, or that use PEM block headers.
    ///
    /// Users of ClusterTrustBundles, including Kubelet, are free to reorder and deduplicate certificate blocks in this file according to their own logic, as well as to drop PEM block headers and inter-block data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_bundle: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::certificates::v1alpha1::ClusterTrustBundleSpec {
    type Optioned = ClusterTrustBundleSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ClusterTrustBundleSpecAc {
    type Optioned = ClusterTrustBundleSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::certificates::v1alpha1::ClusterTrustBundleSpec {
    fn into_optioned(self) -> ClusterTrustBundleSpecAc {
        ClusterTrustBundleSpecAc {
            signer_name: self.signer_name,
            trust_bundle: Some(self.trust_bundle),
        }
    }
    fn try_from_optioned(value: ClusterTrustBundleSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            signer_name: value.signer_name,
            trust_bundle: value
                .trust_bundle
                .ok_or(crate::Error {
                    missing_field: "trust_bundle",
                })?,
        })
    }
    fn merge(&mut self, other: ClusterTrustBundleSpecAc) -> Result<(), crate::Error> {
        if self.signer_name.is_none() {
            self.signer_name = crate::OptionableConvert::try_from_optioned(
                other.signer_name,
            )?;
        } else if let Some(self_value) = self.signer_name.as_mut()
            && let Some(other_value) = other.signer_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.trust_bundle {
            self.trust_bundle = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::certificates::v1alpha1::ClusterTrustBundleSpec,
> for ClusterTrustBundleSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::certificates::v1alpha1::ClusterTrustBundleSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::certificates::v1alpha1::ClusterTrustBundleSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::certificates::v1alpha1::ClusterTrustBundleSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
