pub struct DeviceSelectorOpt {
    pub cel: <Option<
        ::k8s_openapi::api::resource::v1alpha3::CELDeviceSelector,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1alpha3::device_selector::DeviceSelector {
    type Optioned = DeviceSelectorOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceSelectorOpt {
    type Optioned = DeviceSelectorOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha3::device_selector::DeviceSelector {
    fn into_optioned(self) -> DeviceSelectorOpt {
        DeviceSelectorOpt {
            cel: <Option<
                ::k8s_openapi::api::resource::v1alpha3::CELDeviceSelector,
            > as crate::OptionableConvert>::into_optioned(self.cel),
        }
    }
    fn try_from_optioned(
        value: DeviceSelectorOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            cel: <Option<
                ::k8s_openapi::api::resource::v1alpha3::CELDeviceSelector,
            > as crate::OptionableConvert>::try_from_optioned(value.cel)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceSelectorOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::resource::v1alpha3::CELDeviceSelector,
        > as crate::OptionableConvert>::merge(&mut self.cel, other.cel)?;
        Ok(())
    }
}
