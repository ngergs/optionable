pub struct DeviceSelectorOpt {
    pub cel: <Option<
        ::k8s_openapi::api::resource::v1beta1::CELDeviceSelector,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta1::DeviceSelector {
    type Optioned = DeviceSelectorOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceSelectorOpt {
    type Optioned = DeviceSelectorOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1beta1::DeviceSelector {
    fn into_optioned(self) -> DeviceSelectorOpt {
        DeviceSelectorOpt {
            cel: crate::OptionableConvert::into_optioned(self.cel),
        }
    }
    fn try_from_optioned(
        value: DeviceSelectorOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            cel: crate::OptionableConvert::try_from_optioned(value.cel)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceSelectorOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.cel, other.cel)?;
        Ok(())
    }
}
