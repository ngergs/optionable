pub struct ClusterTrustBundleSpecOpt {
    pub signer_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub trust_bundle: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::certificates::v1beta1::ClusterTrustBundleSpec {
    type Optioned = ClusterTrustBundleSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for ClusterTrustBundleSpecOpt {
    type Optioned = ClusterTrustBundleSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::certificates::v1beta1::ClusterTrustBundleSpec {
    fn into_optioned(self) -> ClusterTrustBundleSpecOpt {
        ClusterTrustBundleSpecOpt {
            signer_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.signer_name),
            trust_bundle: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.trust_bundle,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: ClusterTrustBundleSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            signer_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.signer_name)?,
            trust_bundle: <std::string::String as crate::OptionableConvert>::try_from_optioned(
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
        other: ClusterTrustBundleSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.signer_name, other.signer_name)?;
        if let Some(other_value) = other.trust_bundle {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.trust_bundle,
                other_value,
            )?;
        }
        Ok(())
    }
}
