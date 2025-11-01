pub struct ResourceClaimSpecAc {
    pub devices: <Option<
        ::k8s_openapi::api::resource::v1beta2::DeviceClaim,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta2::ResourceClaimSpec {
    type Optioned = ResourceClaimSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimSpecAc {
    type Optioned = ResourceClaimSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta2::ResourceClaimSpec {
    fn into_optioned(self) -> ResourceClaimSpecAc {
        ResourceClaimSpecAc {
            devices: crate::OptionableConvert::into_optioned(self.devices),
        }
    }
    fn try_from_optioned(
        value: ResourceClaimSpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            devices: crate::OptionableConvert::try_from_optioned(value.devices)?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceClaimSpecAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.devices, other.devices)?;
        Ok(())
    }
}
