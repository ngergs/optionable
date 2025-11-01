pub struct DeviceSelectorAc {
    pub cel: <Option<
        ::k8s_openapi::api::resource::v1beta1::CELDeviceSelector,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta1::DeviceSelector {
    type Optioned = DeviceSelectorAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceSelectorAc {
    type Optioned = DeviceSelectorAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1beta1::DeviceSelector {
    fn into_optioned(self) -> DeviceSelectorAc {
        DeviceSelectorAc {
            cel: crate::OptionableConvert::into_optioned(self.cel),
        }
    }
    fn try_from_optioned(
        value: DeviceSelectorAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            cel: crate::OptionableConvert::try_from_optioned(value.cel)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceSelectorAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.cel, other.cel)?;
        Ok(())
    }
}
