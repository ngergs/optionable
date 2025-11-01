pub struct VolumeNodeResourcesAc {
    pub count: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::VolumeNodeResources {
    type Optioned = VolumeNodeResourcesAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeNodeResourcesAc {
    type Optioned = VolumeNodeResourcesAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::storage::v1::VolumeNodeResources {
    fn into_optioned(self) -> VolumeNodeResourcesAc {
        VolumeNodeResourcesAc {
            count: crate::OptionableConvert::into_optioned(self.count),
        }
    }
    fn try_from_optioned(
        value: VolumeNodeResourcesAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            count: crate::OptionableConvert::try_from_optioned(value.count)?,
        })
    }
    fn merge(
        &mut self,
        other: VolumeNodeResourcesAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.count, other.count)?;
        Ok(())
    }
}
