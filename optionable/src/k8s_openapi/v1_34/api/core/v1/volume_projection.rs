#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct VolumeProjectionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_trust_bundle: <Option<
        ::k8s_openapi::api::core::v1::ClusterTrustBundleProjection,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_map: <Option<
        ::k8s_openapi::api::core::v1::ConfigMapProjection,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downward_api: <Option<
        ::k8s_openapi::api::core::v1::DownwardAPIProjection,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_certificate: <Option<
        ::k8s_openapi::api::core::v1::PodCertificateProjection,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: <Option<
        ::k8s_openapi::api::core::v1::SecretProjection,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_token: <Option<
        ::k8s_openapi::api::core::v1::ServiceAccountTokenProjection,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::VolumeProjection {
    type Optioned = VolumeProjectionAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeProjectionAc {
    type Optioned = VolumeProjectionAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::VolumeProjection {
    fn into_optioned(self) -> VolumeProjectionAc {
        VolumeProjectionAc {
            cluster_trust_bundle: crate::OptionableConvert::into_optioned(
                self.cluster_trust_bundle,
            ),
            config_map: crate::OptionableConvert::into_optioned(self.config_map),
            downward_api: crate::OptionableConvert::into_optioned(self.downward_api),
            pod_certificate: crate::OptionableConvert::into_optioned(
                self.pod_certificate,
            ),
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
            pod_certificate: crate::OptionableConvert::try_from_optioned(
                value.pod_certificate,
            )?,
            secret: crate::OptionableConvert::try_from_optioned(value.secret)?,
            service_account_token: crate::OptionableConvert::try_from_optioned(
                value.service_account_token,
            )?,
        })
    }
    fn merge(&mut self, other: VolumeProjectionAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.cluster_trust_bundle,
            other.cluster_trust_bundle,
        )?;
        crate::OptionableConvert::merge(&mut self.config_map, other.config_map)?;
        crate::OptionableConvert::merge(&mut self.downward_api, other.downward_api)?;
        crate::OptionableConvert::merge(
            &mut self.pod_certificate,
            other.pod_certificate,
        )?;
        crate::OptionableConvert::merge(&mut self.secret, other.secret)?;
        crate::OptionableConvert::merge(
            &mut self.service_account_token,
            other.service_account_token,
        )?;
        Ok(())
    }
}
