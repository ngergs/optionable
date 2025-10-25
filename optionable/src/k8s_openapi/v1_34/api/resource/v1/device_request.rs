pub struct DeviceRequestOpt {
    pub exactly: <Option<
        ::k8s_openapi::api::resource::v1::ExactDeviceRequest,
    > as crate::Optionable>::Optioned,
    pub first_available: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceSubRequest>,
    > as crate::Optionable>::Optioned,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::DeviceRequest {
    type Optioned = DeviceRequestOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceRequestOpt {
    type Optioned = DeviceRequestOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1::DeviceRequest {
    fn into_optioned(self) -> DeviceRequestOpt {
        DeviceRequestOpt {
            exactly: <Option<
                ::k8s_openapi::api::resource::v1::ExactDeviceRequest,
            > as crate::OptionableConvert>::into_optioned(self.exactly),
            first_available: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceSubRequest>,
            > as crate::OptionableConvert>::into_optioned(self.first_available),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: DeviceRequestOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            exactly: <Option<
                ::k8s_openapi::api::resource::v1::ExactDeviceRequest,
            > as crate::OptionableConvert>::try_from_optioned(value.exactly)?,
            first_available: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceSubRequest>,
            > as crate::OptionableConvert>::try_from_optioned(value.first_available)?,
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceRequestOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::resource::v1::ExactDeviceRequest,
        > as crate::OptionableConvert>::merge(&mut self.exactly, other.exactly)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceSubRequest>,
        > as crate::OptionableConvert>::merge(
            &mut self.first_available,
            other.first_available,
        )?;
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        Ok(())
    }
}
