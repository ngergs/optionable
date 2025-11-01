pub struct PersistentVolumeClaimVolumeSourceAc {
    pub claim_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::PersistentVolumeClaimVolumeSource {
    type Optioned = PersistentVolumeClaimVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for PersistentVolumeClaimVolumeSourceAc {
    type Optioned = PersistentVolumeClaimVolumeSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::PersistentVolumeClaimVolumeSource {
    fn into_optioned(self) -> PersistentVolumeClaimVolumeSourceAc {
        PersistentVolumeClaimVolumeSourceAc {
            claim_name: Some(crate::OptionableConvert::into_optioned(self.claim_name)),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
        }
    }
    fn try_from_optioned(
        value: PersistentVolumeClaimVolumeSourceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            claim_name: crate::OptionableConvert::try_from_optioned(
                value
                    .claim_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "claim_name",
                    })?,
            )?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
        })
    }
    fn merge(
        &mut self,
        other: PersistentVolumeClaimVolumeSourceAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.claim_name {
            crate::OptionableConvert::merge(&mut self.claim_name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        Ok(())
    }
}
