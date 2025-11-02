#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ClusterTrustBundleSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_bundle: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::certificates::v1alpha1::ClusterTrustBundleSpec {
    type Optioned = ClusterTrustBundleSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ClusterTrustBundleSpecAc {
    type Optioned = ClusterTrustBundleSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::certificates::v1alpha1::ClusterTrustBundleSpec {
    fn into_optioned(self) -> ClusterTrustBundleSpecAc {
        ClusterTrustBundleSpecAc {
            signer_name: crate::OptionableConvert::into_optioned(self.signer_name),
            trust_bundle: Some(
                crate::OptionableConvert::into_optioned(self.trust_bundle),
            ),
        }
    }
    fn try_from_optioned(
        value: ClusterTrustBundleSpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            signer_name: crate::OptionableConvert::try_from_optioned(value.signer_name)?,
            trust_bundle: crate::OptionableConvert::try_from_optioned(
                value
                    .trust_bundle
                    .ok_or(crate::optionable::Error {
                        missing_field: "trust_bundle",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ClusterTrustBundleSpecAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.signer_name, other.signer_name)?;
        if let Some(other_value) = other.trust_bundle {
            crate::OptionableConvert::merge(&mut self.trust_bundle, other_value)?;
        }
        Ok(())
    }
}
