pub struct ExactDeviceRequestOpt {
    pub admin_access: <Option<bool> as crate::Optionable>::Optioned,
    pub allocation_mode: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub capacity: <Option<
        ::k8s_openapi::api::resource::v1::CapacityRequirements,
    > as crate::Optionable>::Optioned,
    pub count: <Option<i64> as crate::Optionable>::Optioned,
    pub device_class_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub selectors: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceSelector>,
    > as crate::Optionable>::Optioned,
    pub tolerations: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceToleration>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::ExactDeviceRequest {
    type Optioned = ExactDeviceRequestOpt;
}
#[automatically_derived]
impl crate::Optionable for ExactDeviceRequestOpt {
    type Optioned = ExactDeviceRequestOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1::ExactDeviceRequest {
    fn into_optioned(self) -> ExactDeviceRequestOpt {
        ExactDeviceRequestOpt {
            admin_access: crate::OptionableConvert::into_optioned(self.admin_access),
            allocation_mode: crate::OptionableConvert::into_optioned(
                self.allocation_mode,
            ),
            capacity: crate::OptionableConvert::into_optioned(self.capacity),
            count: crate::OptionableConvert::into_optioned(self.count),
            device_class_name: Some(
                crate::OptionableConvert::into_optioned(self.device_class_name),
            ),
            selectors: crate::OptionableConvert::into_optioned(self.selectors),
            tolerations: crate::OptionableConvert::into_optioned(self.tolerations),
        }
    }
    fn try_from_optioned(
        value: ExactDeviceRequestOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            admin_access: crate::OptionableConvert::try_from_optioned(
                value.admin_access,
            )?,
            allocation_mode: crate::OptionableConvert::try_from_optioned(
                value.allocation_mode,
            )?,
            capacity: crate::OptionableConvert::try_from_optioned(value.capacity)?,
            count: crate::OptionableConvert::try_from_optioned(value.count)?,
            device_class_name: crate::OptionableConvert::try_from_optioned(
                value
                    .device_class_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "device_class_name",
                    })?,
            )?,
            selectors: crate::OptionableConvert::try_from_optioned(value.selectors)?,
            tolerations: crate::OptionableConvert::try_from_optioned(value.tolerations)?,
        })
    }
    fn merge(
        &mut self,
        other: ExactDeviceRequestOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.admin_access, other.admin_access)?;
        crate::OptionableConvert::merge(
            &mut self.allocation_mode,
            other.allocation_mode,
        )?;
        crate::OptionableConvert::merge(&mut self.capacity, other.capacity)?;
        crate::OptionableConvert::merge(&mut self.count, other.count)?;
        if let Some(other_value) = other.device_class_name {
            crate::OptionableConvert::merge(&mut self.device_class_name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.selectors, other.selectors)?;
        crate::OptionableConvert::merge(&mut self.tolerations, other.tolerations)?;
        Ok(())
    }
}
