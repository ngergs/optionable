#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EphemeralVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_claim_template: <Option<
        ::k8s_openapi027::api::core::v1::PersistentVolumeClaimTemplate,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::EphemeralVolumeSource {
    type Optioned = EphemeralVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for EphemeralVolumeSourceAc {
    type Optioned = EphemeralVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::EphemeralVolumeSource {
    fn into_optioned(self) -> EphemeralVolumeSourceAc {
        EphemeralVolumeSourceAc {
            volume_claim_template: crate::OptionableConvert::into_optioned(
                self.volume_claim_template,
            ),
        }
    }
    fn try_from_optioned(value: EphemeralVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            volume_claim_template: crate::OptionableConvert::try_from_optioned(
                value.volume_claim_template,
            )?,
        })
    }
    fn merge(&mut self, other: EphemeralVolumeSourceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.volume_claim_template,
            other.volume_claim_template,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::EphemeralVolumeSource>
for EphemeralVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::EphemeralVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::EphemeralVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::EphemeralVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
