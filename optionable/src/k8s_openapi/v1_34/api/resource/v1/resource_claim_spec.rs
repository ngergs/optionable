pub struct ResourceClaimSpecOpt {
    pub devices: <Option<
        ::k8s_openapi::api::resource::v1::DeviceClaim,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::ResourceClaimSpec {
    type Optioned = ResourceClaimSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimSpecOpt {
    type Optioned = ResourceClaimSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1::ResourceClaimSpec {
    fn into_optioned(self) -> ResourceClaimSpecOpt {
        ResourceClaimSpecOpt {
            devices: <Option<
                ::k8s_openapi::api::resource::v1::DeviceClaim,
            > as crate::OptionableConvert>::into_optioned(self.devices),
        }
    }
    fn try_from_optioned(
        value: ResourceClaimSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            devices: <Option<
                ::k8s_openapi::api::resource::v1::DeviceClaim,
            > as crate::OptionableConvert>::try_from_optioned(value.devices)?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceClaimSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::resource::v1::DeviceClaim,
        > as crate::OptionableConvert>::merge(&mut self.devices, other.devices)?;
        Ok(())
    }
}
