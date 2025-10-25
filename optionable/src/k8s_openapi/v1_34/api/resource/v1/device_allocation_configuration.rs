pub struct DeviceAllocationConfigurationOpt {
    pub opaque: <Option<
        ::k8s_openapi::api::resource::v1::OpaqueDeviceConfiguration,
    > as crate::Optionable>::Optioned,
    pub requests: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub source: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1::DeviceAllocationConfiguration {
    type Optioned = DeviceAllocationConfigurationOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceAllocationConfigurationOpt {
    type Optioned = DeviceAllocationConfigurationOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1::DeviceAllocationConfiguration {
    fn into_optioned(self) -> DeviceAllocationConfigurationOpt {
        DeviceAllocationConfigurationOpt {
            opaque: <Option<
                ::k8s_openapi::api::resource::v1::OpaqueDeviceConfiguration,
            > as crate::OptionableConvert>::into_optioned(self.opaque),
            requests: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.requests),
            source: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.source,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: DeviceAllocationConfigurationOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            opaque: <Option<
                ::k8s_openapi::api::resource::v1::OpaqueDeviceConfiguration,
            > as crate::OptionableConvert>::try_from_optioned(value.opaque)?,
            requests: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.requests)?,
            source: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .source
                    .ok_or(crate::optionable::Error {
                        missing_field: "source",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceAllocationConfigurationOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::resource::v1::OpaqueDeviceConfiguration,
        > as crate::OptionableConvert>::merge(&mut self.opaque, other.opaque)?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.requests, other.requests)?;
        if let Some(other_value) = other.source {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.source,
                other_value,
            )?;
        }
        Ok(())
    }
}
