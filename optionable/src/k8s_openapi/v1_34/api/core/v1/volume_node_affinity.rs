pub struct VolumeNodeAffinityOpt {
    pub required: <Option<
        ::k8s_openapi::api::core::v1::NodeSelector,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::VolumeNodeAffinity {
    type Optioned = VolumeNodeAffinityOpt;
}
#[automatically_derived]
impl crate::Optionable for VolumeNodeAffinityOpt {
    type Optioned = VolumeNodeAffinityOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::VolumeNodeAffinity {
    fn into_optioned(self) -> VolumeNodeAffinityOpt {
        VolumeNodeAffinityOpt {
            required: crate::OptionableConvert::into_optioned(self.required),
        }
    }
    fn try_from_optioned(
        value: VolumeNodeAffinityOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            required: crate::OptionableConvert::try_from_optioned(value.required)?,
        })
    }
    fn merge(
        &mut self,
        other: VolumeNodeAffinityOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.required, other.required)?;
        Ok(())
    }
}
