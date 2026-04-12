#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Projection that may be projected along with other supported volume types. Exactly one of these fields must be set.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VolumeProjectionAc {
    /// ClusterTrustBundle allows a pod to access the `.spec.trustBundle` field of ClusterTrustBundle objects in an auto-updating file.
    ///
    /// Alpha, gated by the ClusterTrustBundleProjection feature gate.
    ///
    /// ClusterTrustBundle objects can either be selected by name, or by the combination of signer name and a label selector.
    ///
    /// Kubelet performs aggressive normalization of the PEM contents written into the pod filesystem.  Esoteric PEM features such as inter-block comments and block headers are stripped.  Certificates are deduplicated. The ordering of certificates within the file is arbitrary, and Kubelet may change the order over time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_trust_bundle: Option<
        <::k8s_openapi027::api::core::v1::ClusterTrustBundleProjection as crate::Optionable>::Optioned,
    >,
    /// configMap information about the configMap data to project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_map: Option<
        <::k8s_openapi027::api::core::v1::ConfigMapProjection as crate::Optionable>::Optioned,
    >,
    /// downwardAPI information about the downwardAPI data to project
    #[serde(rename = "downwardAPI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downward_api: Option<
        <::k8s_openapi027::api::core::v1::DownwardAPIProjection as crate::Optionable>::Optioned,
    >,
    /// secret information about the secret data to project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<
        <::k8s_openapi027::api::core::v1::SecretProjection as crate::Optionable>::Optioned,
    >,
    /// serviceAccountToken is information about the serviceAccountToken data to project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_token: Option<
        <::k8s_openapi027::api::core::v1::ServiceAccountTokenProjection as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::VolumeProjection {
    type Optioned = VolumeProjectionAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeProjectionAc {
    type Optioned = VolumeProjectionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::VolumeProjection {
    fn into_optioned(self) -> VolumeProjectionAc {
        VolumeProjectionAc {
            cluster_trust_bundle: crate::OptionableConvert::into_optioned(
                self.cluster_trust_bundle,
            ),
            config_map: crate::OptionableConvert::into_optioned(self.config_map),
            downward_api: crate::OptionableConvert::into_optioned(self.downward_api),
            secret: crate::OptionableConvert::into_optioned(self.secret),
            service_account_token: crate::OptionableConvert::into_optioned(
                self.service_account_token,
            ),
        }
    }
    fn try_from_optioned(value: VolumeProjectionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            cluster_trust_bundle: crate::OptionableConvert::try_from_optioned(
                value.cluster_trust_bundle,
            )?,
            config_map: crate::OptionableConvert::try_from_optioned(value.config_map)?,
            downward_api: crate::OptionableConvert::try_from_optioned(
                value.downward_api,
            )?,
            secret: crate::OptionableConvert::try_from_optioned(value.secret)?,
            service_account_token: crate::OptionableConvert::try_from_optioned(
                value.service_account_token,
            )?,
        })
    }
    fn merge(&mut self, other: VolumeProjectionAc) -> Result<(), crate::Error> {
        if self.cluster_trust_bundle.is_none() {
            self.cluster_trust_bundle = other.cluster_trust_bundle;
        }
        if let Some(other_value) = other.cluster_trust_bundle {
            crate::OptionableConvert::merge(
                &mut self.cluster_trust_bundle,
                other_value,
            )?;
        }
        if self.config_map.is_none() {
            self.config_map = other.config_map;
        }
        if let Some(other_value) = other.config_map {
            crate::OptionableConvert::merge(&mut self.config_map, other_value)?;
        }
        if self.downward_api.is_none() {
            self.downward_api = other.downward_api;
        }
        if let Some(other_value) = other.downward_api {
            crate::OptionableConvert::merge(&mut self.downward_api, other_value)?;
        }
        if self.secret.is_none() {
            self.secret = other.secret;
        }
        if let Some(other_value) = other.secret {
            crate::OptionableConvert::merge(&mut self.secret, other_value)?;
        }
        if self.service_account_token.is_none() {
            self.service_account_token = other.service_account_token;
        }
        if let Some(other_value) = other.service_account_token {
            crate::OptionableConvert::merge(
                &mut self.service_account_token,
                other_value,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::VolumeProjection>
for VolumeProjectionAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::VolumeProjection) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::VolumeProjection, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::VolumeProjection,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
