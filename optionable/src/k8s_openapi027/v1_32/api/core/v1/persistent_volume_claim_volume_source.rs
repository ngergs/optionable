#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PersistentVolumeClaimVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::core::v1::PersistentVolumeClaimVolumeSource {
    type Optioned = PersistentVolumeClaimVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for PersistentVolumeClaimVolumeSourceAc {
    type Optioned = PersistentVolumeClaimVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::PersistentVolumeClaimVolumeSource {
    fn into_optioned(self) -> PersistentVolumeClaimVolumeSourceAc {
        PersistentVolumeClaimVolumeSourceAc {
            claim_name: Some(crate::OptionableConvert::into_optioned(self.claim_name)),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
        }
    }
    fn try_from_optioned(
        value: PersistentVolumeClaimVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            claim_name: crate::OptionableConvert::try_from_optioned(
                value
                    .claim_name
                    .ok_or(crate::Error {
                        missing_field: "claim_name",
                    })?,
            )?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
        })
    }
    fn merge(
        &mut self,
        other: PersistentVolumeClaimVolumeSourceAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.claim_name {
            crate::OptionableConvert::merge(&mut self.claim_name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::core::v1::PersistentVolumeClaimVolumeSource,
> for PersistentVolumeClaimVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::PersistentVolumeClaimVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::PersistentVolumeClaimVolumeSource,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PersistentVolumeClaimVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
