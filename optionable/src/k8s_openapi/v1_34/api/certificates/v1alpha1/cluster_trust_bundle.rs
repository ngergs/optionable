pub struct ClusterTrustBundleOpt {
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub spec: Option<
        <::k8s_openapi::api::certificates::v1alpha1::ClusterTrustBundleSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::certificates::v1alpha1::ClusterTrustBundle {
    type Optioned = ClusterTrustBundleOpt;
}
#[automatically_derived]
impl crate::Optionable for ClusterTrustBundleOpt {
    type Optioned = ClusterTrustBundleOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::certificates::v1alpha1::ClusterTrustBundle {
    fn into_optioned(self) -> ClusterTrustBundleOpt {
        ClusterTrustBundleOpt {
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            spec: Some(
                <::k8s_openapi::api::certificates::v1alpha1::ClusterTrustBundleSpec as crate::OptionableConvert>::into_optioned(
                    self.spec,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: ClusterTrustBundleOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            spec: <::k8s_openapi::api::certificates::v1alpha1::ClusterTrustBundleSpec as crate::OptionableConvert>::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::optionable::Error {
                        missing_field: "spec",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ClusterTrustBundleOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        if let Some(other_value) = other.spec {
            <::k8s_openapi::api::certificates::v1alpha1::ClusterTrustBundleSpec as crate::OptionableConvert>::merge(
                &mut self.spec,
                other_value,
            )?;
        }
        Ok(())
    }
}
