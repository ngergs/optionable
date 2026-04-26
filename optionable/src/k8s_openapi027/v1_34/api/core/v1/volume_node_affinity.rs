#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// VolumeNodeAffinity defines constraints that limit what nodes this volume can be accessed from.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VolumeNodeAffinityAc {
    /// required specifies hard node constraints that must be met.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<
        <::k8s_openapi027::api::core::v1::NodeSelector as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::VolumeNodeAffinity {
    type Optioned = VolumeNodeAffinityAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeNodeAffinityAc {
    type Optioned = VolumeNodeAffinityAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::VolumeNodeAffinity {
    fn into_optioned(self) -> VolumeNodeAffinityAc {
        VolumeNodeAffinityAc {
            required: crate::OptionableConvert::into_optioned(self.required),
        }
    }
    fn try_from_optioned(value: VolumeNodeAffinityAc) -> Result<Self, crate::Error> {
        Ok(Self {
            required: crate::OptionableConvert::try_from_optioned(value.required)?,
        })
    }
    fn merge(&mut self, other: VolumeNodeAffinityAc) -> Result<(), crate::Error> {
        if self.required.is_none() {
            self.required = crate::OptionableConvert::try_from_optioned(other.required)?;
        } else if let Some(self_value) = self.required.as_mut()
            && let Some(other_value) = other.required
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::VolumeNodeAffinity>
for VolumeNodeAffinityAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::VolumeNodeAffinity,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::VolumeNodeAffinity, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::VolumeNodeAffinity,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for VolumeNodeAffinityAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.required, other.required);
    }
}
