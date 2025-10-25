pub struct DeviceSubRequestOpt {
    pub allocation_mode: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub capacity: <Option<
        ::k8s_openapi::api::resource::v1::CapacityRequirements,
    > as crate::Optionable>::Optioned,
    pub count: <Option<i64> as crate::Optionable>::Optioned,
    pub device_class_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub selectors: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceSelector>,
    > as crate::Optionable>::Optioned,
    pub tolerations: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceToleration>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::DeviceSubRequest {
    type Optioned = DeviceSubRequestOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceSubRequestOpt {
    type Optioned = DeviceSubRequestOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1::DeviceSubRequest {
    fn into_optioned(self) -> DeviceSubRequestOpt {
        DeviceSubRequestOpt {
            allocation_mode: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.allocation_mode),
            capacity: <Option<
                ::k8s_openapi::api::resource::v1::CapacityRequirements,
            > as crate::OptionableConvert>::into_optioned(self.capacity),
            count: <Option<i64> as crate::OptionableConvert>::into_optioned(self.count),
            device_class_name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.device_class_name,
                ),
            ),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            selectors: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceSelector>,
            > as crate::OptionableConvert>::into_optioned(self.selectors),
            tolerations: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceToleration>,
            > as crate::OptionableConvert>::into_optioned(self.tolerations),
        }
    }
    fn try_from_optioned(
        value: DeviceSubRequestOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            allocation_mode: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.allocation_mode)?,
            capacity: <Option<
                ::k8s_openapi::api::resource::v1::CapacityRequirements,
            > as crate::OptionableConvert>::try_from_optioned(value.capacity)?,
            count: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(value.count)?,
            device_class_name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .device_class_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "device_class_name",
                    })?,
            )?,
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            selectors: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceSelector>,
            > as crate::OptionableConvert>::try_from_optioned(value.selectors)?,
            tolerations: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceToleration>,
            > as crate::OptionableConvert>::try_from_optioned(value.tolerations)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceSubRequestOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.allocation_mode,
            other.allocation_mode,
        )?;
        <Option<
            ::k8s_openapi::api::resource::v1::CapacityRequirements,
        > as crate::OptionableConvert>::merge(&mut self.capacity, other.capacity)?;
        <Option<i64> as crate::OptionableConvert>::merge(&mut self.count, other.count)?;
        if let Some(other_value) = other.device_class_name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.device_class_name,
                other_value,
            )?;
        }
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        <Option<
            std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceSelector>,
        > as crate::OptionableConvert>::merge(&mut self.selectors, other.selectors)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceToleration>,
        > as crate::OptionableConvert>::merge(&mut self.tolerations, other.tolerations)?;
        Ok(())
    }
}
