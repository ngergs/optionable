pub struct DeviceClaimAc {
    pub config: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1beta2::DeviceClaimConfiguration>,
    > as crate::Optionable>::Optioned,
    pub constraints: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1beta2::DeviceConstraint>,
    > as crate::Optionable>::Optioned,
    pub requests: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1beta2::DeviceRequest>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta2::DeviceClaim {
    type Optioned = DeviceClaimAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceClaimAc {
    type Optioned = DeviceClaimAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1beta2::DeviceClaim {
    fn into_optioned(self) -> DeviceClaimAc {
        DeviceClaimAc {
            config: crate::OptionableConvert::into_optioned(self.config),
            constraints: crate::OptionableConvert::into_optioned(self.constraints),
            requests: crate::OptionableConvert::into_optioned(self.requests),
        }
    }
    fn try_from_optioned(
        value: DeviceClaimAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            config: crate::OptionableConvert::try_from_optioned(value.config)?,
            constraints: crate::OptionableConvert::try_from_optioned(value.constraints)?,
            requests: crate::OptionableConvert::try_from_optioned(value.requests)?,
        })
    }
    fn merge(&mut self, other: DeviceClaimAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.config, other.config)?;
        crate::OptionableConvert::merge(&mut self.constraints, other.constraints)?;
        crate::OptionableConvert::merge(&mut self.requests, other.requests)?;
        Ok(())
    }
}
