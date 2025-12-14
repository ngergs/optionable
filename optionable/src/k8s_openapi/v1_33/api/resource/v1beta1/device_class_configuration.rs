#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceClassConfigurationAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opaque: <Option<
        ::k8s_openapi::api::resource::v1beta1::OpaqueDeviceConfiguration,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1beta1::DeviceClassConfiguration {
    type Optioned = DeviceClassConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceClassConfigurationAc {
    type Optioned = DeviceClassConfigurationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta1::DeviceClassConfiguration {
    fn into_optioned(self) -> DeviceClassConfigurationAc {
        DeviceClassConfigurationAc {
            opaque: crate::OptionableConvert::into_optioned(self.opaque),
        }
    }
    fn try_from_optioned(
        value: DeviceClassConfigurationAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            opaque: crate::OptionableConvert::try_from_optioned(value.opaque)?,
        })
    }
    fn merge(&mut self, other: DeviceClassConfigurationAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.opaque, other.opaque)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::api::resource::v1beta1::DeviceClassConfiguration,
> for DeviceClassConfigurationAc {
    fn from_optionable(
        value: ::k8s_openapi::api::resource::v1beta1::DeviceClassConfiguration,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::resource::v1beta1::DeviceClassConfiguration,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::resource::v1beta1::DeviceClassConfiguration,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
