#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ClusterTrustBundleProjection describes how to select a set of ClusterTrustBundle objects and project their contents into the pod filesystem.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClusterTrustBundleProjectionAc {
    /// Select all ClusterTrustBundles that match this label selector.  Only has effect if signerName is set.  Mutually-exclusive with name.  If unset, interpreted as "match nothing".  If set but empty, interpreted as "match everything".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    /// Select a single ClusterTrustBundle by object name.  Mutually-exclusive with signerName and labelSelector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// If true, don't block pod startup if the referenced ClusterTrustBundle(s) aren't available.  If using name, then the named ClusterTrustBundle is allowed not to exist.  If using signerName, then the combination of signerName and labelSelector is allowed to match zero ClusterTrustBundles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    /// Relative path from the volume root to write the bundle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
    /// Select all ClusterTrustBundles that match this signer name. Mutually-exclusive with name.  The contents of all selected ClusterTrustBundles will be unified and deduplicated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ClusterTrustBundleProjection {
    type Optioned = ClusterTrustBundleProjectionAc;
}
#[automatically_derived]
impl crate::Optionable for ClusterTrustBundleProjectionAc {
    type Optioned = ClusterTrustBundleProjectionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::ClusterTrustBundleProjection {
    fn into_optioned(self) -> ClusterTrustBundleProjectionAc {
        ClusterTrustBundleProjectionAc {
            label_selector: crate::OptionableConvert::into_optioned(self.label_selector),
            name: self.name,
            optional: self.optional,
            path: Some(self.path),
            signer_name: self.signer_name,
        }
    }
    fn try_from_optioned(
        value: ClusterTrustBundleProjectionAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            label_selector: crate::OptionableConvert::try_from_optioned(
                value.label_selector,
            )?,
            name: value.name,
            optional: value.optional,
            path: value
                .path
                .ok_or(crate::Error {
                    missing_field: "path",
                })?,
            signer_name: value.signer_name,
        })
    }
    fn merge(
        &mut self,
        other: ClusterTrustBundleProjectionAc,
    ) -> Result<(), crate::Error> {
        if self.label_selector.is_none() {
            self.label_selector = other.label_selector;
        }
        if let Some(other_value) = other.label_selector {
            crate::OptionableConvert::merge(&mut self.label_selector, other_value)?;
        }
        if self.name.is_none() {
            self.name = other.name;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        if self.optional.is_none() {
            self.optional = other.optional;
        }
        if let Some(other_value) = other.optional {
            crate::OptionableConvert::merge(&mut self.optional, other_value)?;
        }
        if let Some(other_value) = other.path {
            self.path = other_value;
        }
        if self.signer_name.is_none() {
            self.signer_name = other.signer_name;
        }
        if let Some(other_value) = other.signer_name {
            crate::OptionableConvert::merge(&mut self.signer_name, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ClusterTrustBundleProjection>
for ClusterTrustBundleProjectionAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ClusterTrustBundleProjection,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::ClusterTrustBundleProjection,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ClusterTrustBundleProjection,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
