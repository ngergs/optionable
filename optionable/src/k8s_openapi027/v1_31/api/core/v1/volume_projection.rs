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
            self.cluster_trust_bundle = crate::OptionableConvert::try_from_optioned(
                other.cluster_trust_bundle,
            )?;
        } else if let Some(self_value) = self.cluster_trust_bundle.as_mut()
            && let Some(other_value) = other.cluster_trust_bundle
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.config_map.is_none() {
            self.config_map = crate::OptionableConvert::try_from_optioned(
                other.config_map,
            )?;
        } else if let Some(self_value) = self.config_map.as_mut()
            && let Some(other_value) = other.config_map
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.downward_api.is_none() {
            self.downward_api = crate::OptionableConvert::try_from_optioned(
                other.downward_api,
            )?;
        } else if let Some(self_value) = self.downward_api.as_mut()
            && let Some(other_value) = other.downward_api
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.secret.is_none() {
            self.secret = crate::OptionableConvert::try_from_optioned(other.secret)?;
        } else if let Some(self_value) = self.secret.as_mut()
            && let Some(other_value) = other.secret
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.service_account_token.is_none() {
            self.service_account_token = crate::OptionableConvert::try_from_optioned(
                other.service_account_token,
            )?;
        } else if let Some(self_value) = self.service_account_token.as_mut()
            && let Some(other_value) = other.service_account_token
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
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
impl k8s_openapi027::DeepMerge for VolumeProjectionAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.cluster_trust_bundle,
            other.cluster_trust_bundle,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.config_map, other.config_map);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.downward_api,
            other.downward_api,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.secret, other.secret);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.service_account_token,
            other.service_account_token,
        );
    }
}
