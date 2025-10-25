pub struct DeviceTaintSelectorOpt {
    pub device: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub device_class_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub driver: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub pool: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub selectors: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1alpha3::DeviceSelector>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha3::DeviceTaintSelector {
    type Optioned = DeviceTaintSelectorOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceTaintSelectorOpt {
    type Optioned = DeviceTaintSelectorOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha3::DeviceTaintSelector {
    fn into_optioned(self) -> DeviceTaintSelectorOpt {
        DeviceTaintSelectorOpt {
            device: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.device),
            device_class_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.device_class_name),
            driver: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.driver),
            pool: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.pool),
            selectors: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1alpha3::DeviceSelector>,
            > as crate::OptionableConvert>::into_optioned(self.selectors),
        }
    }
    fn try_from_optioned(
        value: DeviceTaintSelectorOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            device: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.device)?,
            device_class_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.device_class_name)?,
            driver: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.driver)?,
            pool: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.pool)?,
            selectors: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1alpha3::DeviceSelector>,
            > as crate::OptionableConvert>::try_from_optioned(value.selectors)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceTaintSelectorOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.device, other.device)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.device_class_name,
            other.device_class_name,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.driver, other.driver)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.pool, other.pool)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::resource::v1alpha3::DeviceSelector>,
        > as crate::OptionableConvert>::merge(&mut self.selectors, other.selectors)?;
        Ok(())
    }
}
