pub struct DeviceClaimOpt {
    pub config: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1beta1::DeviceClaimConfiguration>,
    > as crate::Optionable>::Optioned,
    pub constraints: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1beta1::DeviceConstraint>,
    > as crate::Optionable>::Optioned,
    pub requests: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1beta1::DeviceRequest>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta1::DeviceClaim {
    type Optioned = DeviceClaimOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceClaimOpt {
    type Optioned = DeviceClaimOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1beta1::DeviceClaim {
    fn into_optioned(self) -> DeviceClaimOpt {
        DeviceClaimOpt {
            config: crate::OptionableConvert::into_optioned(self.config),
            constraints: crate::OptionableConvert::into_optioned(self.constraints),
            requests: crate::OptionableConvert::into_optioned(self.requests),
        }
    }
    fn try_from_optioned(
        value: DeviceClaimOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            config: crate::OptionableConvert::try_from_optioned(value.config)?,
            constraints: crate::OptionableConvert::try_from_optioned(value.constraints)?,
            requests: crate::OptionableConvert::try_from_optioned(value.requests)?,
        })
    }
    fn merge(&mut self, other: DeviceClaimOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.config, other.config)?;
        crate::OptionableConvert::merge(&mut self.constraints, other.constraints)?;
        crate::OptionableConvert::merge(&mut self.requests, other.requests)?;
        Ok(())
    }
}
