pub struct DeviceTaintSelectorAc {
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
    type Optioned = DeviceTaintSelectorAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceTaintSelectorAc {
    type Optioned = DeviceTaintSelectorAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha3::DeviceTaintSelector {
    fn into_optioned(self) -> DeviceTaintSelectorAc {
        DeviceTaintSelectorAc {
            device: crate::OptionableConvert::into_optioned(self.device),
            device_class_name: crate::OptionableConvert::into_optioned(
                self.device_class_name,
            ),
            driver: crate::OptionableConvert::into_optioned(self.driver),
            pool: crate::OptionableConvert::into_optioned(self.pool),
            selectors: crate::OptionableConvert::into_optioned(self.selectors),
        }
    }
    fn try_from_optioned(
        value: DeviceTaintSelectorAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            device: crate::OptionableConvert::try_from_optioned(value.device)?,
            device_class_name: crate::OptionableConvert::try_from_optioned(
                value.device_class_name,
            )?,
            driver: crate::OptionableConvert::try_from_optioned(value.driver)?,
            pool: crate::OptionableConvert::try_from_optioned(value.pool)?,
            selectors: crate::OptionableConvert::try_from_optioned(value.selectors)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceTaintSelectorAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.device, other.device)?;
        crate::OptionableConvert::merge(
            &mut self.device_class_name,
            other.device_class_name,
        )?;
        crate::OptionableConvert::merge(&mut self.driver, other.driver)?;
        crate::OptionableConvert::merge(&mut self.pool, other.pool)?;
        crate::OptionableConvert::merge(&mut self.selectors, other.selectors)?;
        Ok(())
    }
}
