#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct VolumeNodeAffinityAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: <Option<
        ::k8s_openapi::api::core::v1::NodeSelector,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::VolumeNodeAffinity {
    type Optioned = VolumeNodeAffinityAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeNodeAffinityAc {
    type Optioned = VolumeNodeAffinityAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::VolumeNodeAffinity {
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
        crate::OptionableConvert::merge(&mut self.required, other.required)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::VolumeNodeAffinity>
for VolumeNodeAffinityAc {
    fn from_optionable(value: ::k8s_openapi::api::core::v1::VolumeNodeAffinity) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::VolumeNodeAffinity, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::VolumeNodeAffinity,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
