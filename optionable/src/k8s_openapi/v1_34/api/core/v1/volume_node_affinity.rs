pub struct VolumeNodeAffinityAc {
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
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::VolumeNodeAffinity {
    fn into_optioned(self) -> VolumeNodeAffinityAc {
        VolumeNodeAffinityAc {
            required: crate::OptionableConvert::into_optioned(self.required),
        }
    }
    fn try_from_optioned(
        value: VolumeNodeAffinityAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            required: crate::OptionableConvert::try_from_optioned(value.required)?,
        })
    }
    fn merge(
        &mut self,
        other: VolumeNodeAffinityAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.required, other.required)?;
        Ok(())
    }
}
