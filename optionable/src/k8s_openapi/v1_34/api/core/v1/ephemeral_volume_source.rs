pub struct EphemeralVolumeSourceAc {
    pub volume_claim_template: <Option<
        ::k8s_openapi::api::core::v1::PersistentVolumeClaimTemplate,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::EphemeralVolumeSource {
    type Optioned = EphemeralVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for EphemeralVolumeSourceAc {
    type Optioned = EphemeralVolumeSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::EphemeralVolumeSource {
    fn into_optioned(self) -> EphemeralVolumeSourceAc {
        EphemeralVolumeSourceAc {
            volume_claim_template: crate::OptionableConvert::into_optioned(
                self.volume_claim_template,
            ),
        }
    }
    fn try_from_optioned(
        value: EphemeralVolumeSourceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            volume_claim_template: crate::OptionableConvert::try_from_optioned(
                value.volume_claim_template,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: EphemeralVolumeSourceAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.volume_claim_template,
            other.volume_claim_template,
        )?;
        Ok(())
    }
}
