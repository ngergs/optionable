pub struct VolumeProjectionOpt {
    pub cluster_trust_bundle: <Option<
        ::k8s_openapi::api::core::v1::ClusterTrustBundleProjection,
    > as crate::Optionable>::Optioned,
    pub config_map: <Option<
        ::k8s_openapi::api::core::v1::ConfigMapProjection,
    > as crate::Optionable>::Optioned,
    pub downward_api: <Option<
        ::k8s_openapi::api::core::v1::DownwardAPIProjection,
    > as crate::Optionable>::Optioned,
    pub pod_certificate: <Option<
        ::k8s_openapi::api::core::v1::PodCertificateProjection,
    > as crate::Optionable>::Optioned,
    pub secret: <Option<
        ::k8s_openapi::api::core::v1::SecretProjection,
    > as crate::Optionable>::Optioned,
    pub service_account_token: <Option<
        ::k8s_openapi::api::core::v1::ServiceAccountTokenProjection,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::volume_projection::VolumeProjection {
    type Optioned = VolumeProjectionOpt;
}
#[automatically_derived]
impl crate::Optionable for VolumeProjectionOpt {
    type Optioned = VolumeProjectionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::volume_projection::VolumeProjection {
    fn into_optioned(self) -> VolumeProjectionOpt {
        VolumeProjectionOpt {
            cluster_trust_bundle: <Option<
                ::k8s_openapi::api::core::v1::ClusterTrustBundleProjection,
            > as crate::OptionableConvert>::into_optioned(self.cluster_trust_bundle),
            config_map: <Option<
                ::k8s_openapi::api::core::v1::ConfigMapProjection,
            > as crate::OptionableConvert>::into_optioned(self.config_map),
            downward_api: <Option<
                ::k8s_openapi::api::core::v1::DownwardAPIProjection,
            > as crate::OptionableConvert>::into_optioned(self.downward_api),
            pod_certificate: <Option<
                ::k8s_openapi::api::core::v1::PodCertificateProjection,
            > as crate::OptionableConvert>::into_optioned(self.pod_certificate),
            secret: <Option<
                ::k8s_openapi::api::core::v1::SecretProjection,
            > as crate::OptionableConvert>::into_optioned(self.secret),
            service_account_token: <Option<
                ::k8s_openapi::api::core::v1::ServiceAccountTokenProjection,
            > as crate::OptionableConvert>::into_optioned(self.service_account_token),
        }
    }
    fn try_from_optioned(
        value: VolumeProjectionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            cluster_trust_bundle: <Option<
                ::k8s_openapi::api::core::v1::ClusterTrustBundleProjection,
            > as crate::OptionableConvert>::try_from_optioned(
                value.cluster_trust_bundle,
            )?,
            config_map: <Option<
                ::k8s_openapi::api::core::v1::ConfigMapProjection,
            > as crate::OptionableConvert>::try_from_optioned(value.config_map)?,
            downward_api: <Option<
                ::k8s_openapi::api::core::v1::DownwardAPIProjection,
            > as crate::OptionableConvert>::try_from_optioned(value.downward_api)?,
            pod_certificate: <Option<
                ::k8s_openapi::api::core::v1::PodCertificateProjection,
            > as crate::OptionableConvert>::try_from_optioned(value.pod_certificate)?,
            secret: <Option<
                ::k8s_openapi::api::core::v1::SecretProjection,
            > as crate::OptionableConvert>::try_from_optioned(value.secret)?,
            service_account_token: <Option<
                ::k8s_openapi::api::core::v1::ServiceAccountTokenProjection,
            > as crate::OptionableConvert>::try_from_optioned(
                value.service_account_token,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: VolumeProjectionOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::core::v1::ClusterTrustBundleProjection,
        > as crate::OptionableConvert>::merge(
            &mut self.cluster_trust_bundle,
            other.cluster_trust_bundle,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::ConfigMapProjection,
        > as crate::OptionableConvert>::merge(&mut self.config_map, other.config_map)?;
        <Option<
            ::k8s_openapi::api::core::v1::DownwardAPIProjection,
        > as crate::OptionableConvert>::merge(
            &mut self.downward_api,
            other.downward_api,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::PodCertificateProjection,
        > as crate::OptionableConvert>::merge(
            &mut self.pod_certificate,
            other.pod_certificate,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::SecretProjection,
        > as crate::OptionableConvert>::merge(&mut self.secret, other.secret)?;
        <Option<
            ::k8s_openapi::api::core::v1::ServiceAccountTokenProjection,
        > as crate::OptionableConvert>::merge(
            &mut self.service_account_token,
            other.service_account_token,
        )?;
        Ok(())
    }
}
