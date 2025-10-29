pub struct VolumeNodeResourcesOpt {
    pub count: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::VolumeNodeResources {
    type Optioned = VolumeNodeResourcesOpt;
}
#[automatically_derived]
impl crate::Optionable for VolumeNodeResourcesOpt {
    type Optioned = VolumeNodeResourcesOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::storage::v1::VolumeNodeResources {
    fn into_optioned(self) -> VolumeNodeResourcesOpt {
        VolumeNodeResourcesOpt {
            count: crate::OptionableConvert::into_optioned(self.count),
        }
    }
    fn try_from_optioned(
        value: VolumeNodeResourcesOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            count: crate::OptionableConvert::try_from_optioned(value.count)?,
        })
    }
    fn merge(
        &mut self,
        other: VolumeNodeResourcesOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.count, other.count)?;
        Ok(())
    }
}
