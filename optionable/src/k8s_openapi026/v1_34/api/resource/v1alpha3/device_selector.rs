#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceSelectorAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cel: <Option<
        ::k8s_openapi026::api::resource::v1alpha3::CELDeviceSelector,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::resource::v1alpha3::DeviceSelector {
    type Optioned = DeviceSelectorAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceSelectorAc {
    type Optioned = DeviceSelectorAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::resource::v1alpha3::DeviceSelector {
    fn into_optioned(self) -> DeviceSelectorAc {
        DeviceSelectorAc {
            cel: crate::OptionableConvert::into_optioned(self.cel),
        }
    }
    fn try_from_optioned(value: DeviceSelectorAc) -> Result<Self, crate::Error> {
        Ok(Self {
            cel: crate::OptionableConvert::try_from_optioned(value.cel)?,
        })
    }
    fn merge(&mut self, other: DeviceSelectorAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.cel, other.cel)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::resource::v1alpha3::DeviceSelector>
for DeviceSelectorAc {
    fn from_optionable(
        value: k8s_openapi026::api::resource::v1alpha3::DeviceSelector,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::resource::v1alpha3::DeviceSelector, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::resource::v1alpha3::DeviceSelector,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
