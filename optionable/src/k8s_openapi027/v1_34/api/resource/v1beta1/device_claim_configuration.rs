#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceClaimConfigurationAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opaque: <Option<
        ::k8s_openapi027::api::resource::v1beta1::OpaqueDeviceConfiguration,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::resource::v1beta1::DeviceClaimConfiguration {
    type Optioned = DeviceClaimConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceClaimConfigurationAc {
    type Optioned = DeviceClaimConfigurationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1beta1::DeviceClaimConfiguration {
    fn into_optioned(self) -> DeviceClaimConfigurationAc {
        DeviceClaimConfigurationAc {
            opaque: crate::OptionableConvert::into_optioned(self.opaque),
            requests: crate::OptionableConvert::into_optioned(self.requests),
        }
    }
    fn try_from_optioned(
        value: DeviceClaimConfigurationAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            opaque: crate::OptionableConvert::try_from_optioned(value.opaque)?,
            requests: crate::OptionableConvert::try_from_optioned(value.requests)?,
        })
    }
    fn merge(&mut self, other: DeviceClaimConfigurationAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.opaque, other.opaque)?;
        crate::OptionableConvert::merge(&mut self.requests, other.requests)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::resource::v1beta1::DeviceClaimConfiguration,
> for DeviceClaimConfigurationAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta1::DeviceClaimConfiguration,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1beta1::DeviceClaimConfiguration,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta1::DeviceClaimConfiguration,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
