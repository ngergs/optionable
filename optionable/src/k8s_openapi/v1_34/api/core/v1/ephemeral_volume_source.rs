pub struct EphemeralVolumeSourceOpt {
    pub volume_claim_template: <Option<
        ::k8s_openapi::api::core::v1::PersistentVolumeClaimTemplate,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::ephemeral_volume_source::EphemeralVolumeSource {
    type Optioned = EphemeralVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for EphemeralVolumeSourceOpt {
    type Optioned = EphemeralVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::ephemeral_volume_source::EphemeralVolumeSource {
    fn into_optioned(self) -> EphemeralVolumeSourceOpt {
        EphemeralVolumeSourceOpt {
            volume_claim_template: <Option<
                ::k8s_openapi::api::core::v1::PersistentVolumeClaimTemplate,
            > as crate::OptionableConvert>::into_optioned(self.volume_claim_template),
        }
    }
    fn try_from_optioned(
        value: EphemeralVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            volume_claim_template: <Option<
                ::k8s_openapi::api::core::v1::PersistentVolumeClaimTemplate,
            > as crate::OptionableConvert>::try_from_optioned(
                value.volume_claim_template,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: EphemeralVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::core::v1::PersistentVolumeClaimTemplate,
        > as crate::OptionableConvert>::merge(
            &mut self.volume_claim_template,
            other.volume_claim_template,
        )?;
        Ok(())
    }
}
