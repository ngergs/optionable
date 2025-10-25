pub struct DeviceClaimOpt {
    pub config: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceClaimConfiguration>,
    > as crate::Optionable>::Optioned,
    pub constraints: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceConstraint>,
    > as crate::Optionable>::Optioned,
    pub requests: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceRequest>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::device_claim::DeviceClaim {
    type Optioned = DeviceClaimOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceClaimOpt {
    type Optioned = DeviceClaimOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1::device_claim::DeviceClaim {
    fn into_optioned(self) -> DeviceClaimOpt {
        DeviceClaimOpt {
            config: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceClaimConfiguration>,
            > as crate::OptionableConvert>::into_optioned(self.config),
            constraints: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceConstraint>,
            > as crate::OptionableConvert>::into_optioned(self.constraints),
            requests: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceRequest>,
            > as crate::OptionableConvert>::into_optioned(self.requests),
        }
    }
    fn try_from_optioned(
        value: DeviceClaimOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            config: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceClaimConfiguration>,
            > as crate::OptionableConvert>::try_from_optioned(value.config)?,
            constraints: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceConstraint>,
            > as crate::OptionableConvert>::try_from_optioned(value.constraints)?,
            requests: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceRequest>,
            > as crate::OptionableConvert>::try_from_optioned(value.requests)?,
        })
    }
    fn merge(&mut self, other: DeviceClaimOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceClaimConfiguration>,
        > as crate::OptionableConvert>::merge(&mut self.config, other.config)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceConstraint>,
        > as crate::OptionableConvert>::merge(&mut self.constraints, other.constraints)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceRequest>,
        > as crate::OptionableConvert>::merge(&mut self.requests, other.requests)?;
        Ok(())
    }
}
