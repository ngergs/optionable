pub struct PersistentVolumeClaimVolumeSourceOpt {
    pub claim_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::persistent_volume_claim_volume_source::PersistentVolumeClaimVolumeSource {
    type Optioned = PersistentVolumeClaimVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for PersistentVolumeClaimVolumeSourceOpt {
    type Optioned = PersistentVolumeClaimVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::persistent_volume_claim_volume_source::PersistentVolumeClaimVolumeSource {
    fn into_optioned(self) -> PersistentVolumeClaimVolumeSourceOpt {
        PersistentVolumeClaimVolumeSourceOpt {
            claim_name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.claim_name,
                ),
            ),
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.read_only),
        }
    }
    fn try_from_optioned(
        value: PersistentVolumeClaimVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            claim_name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .claim_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "claim_name",
                    })?,
            )?,
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.read_only)?,
        })
    }
    fn merge(
        &mut self,
        other: PersistentVolumeClaimVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.claim_name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.claim_name,
                other_value,
            )?;
        }
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.read_only, other.read_only)?;
        Ok(())
    }
}
