#[derive(kube::Resource)]
#[resource(inherit = ClusterTrustBundle)]
pub struct ClusterTrustBundleAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub spec: Option<
        <::k8s_openapi::api::certificates::v1alpha1::ClusterTrustBundleSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::certificates::v1alpha1::ClusterTrustBundle {
    type Optioned = ClusterTrustBundleAc;
}
#[automatically_derived]
impl crate::Optionable for ClusterTrustBundleAc {
    type Optioned = ClusterTrustBundleAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::certificates::v1alpha1::ClusterTrustBundle {
    fn into_optioned(self) -> ClusterTrustBundleAc {
        ClusterTrustBundleAc {
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
        }
    }
    fn try_from_optioned(
        value: ClusterTrustBundleAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(
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
        other: ClusterTrustBundleAc,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::certificates::v1alpha1::ClusterTrustBundle;
