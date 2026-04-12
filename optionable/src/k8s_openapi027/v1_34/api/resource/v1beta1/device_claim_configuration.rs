#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DeviceClaimConfiguration is used for configuration parameters in DeviceClaim.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceClaimConfigurationAc {
    /// Opaque provides driver-specific configuration parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opaque: Option<
        <::k8s_openapi027::api::resource::v1beta1::OpaqueDeviceConfiguration as crate::Optionable>::Optioned,
    >,
    /// Requests lists the names of requests where the configuration applies. If empty, it applies to all requests.
    ///
    /// References to subrequests must include the name of the main request and may include the subrequest using the format \<main request\>\[/\<subrequest\>\]. If just the main request is given, the configuration applies to all subrequests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<std::vec::Vec<std::string::String>>,
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
            requests: self.requests,
        }
    }
    fn try_from_optioned(
        value: DeviceClaimConfigurationAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            opaque: crate::OptionableConvert::try_from_optioned(value.opaque)?,
            requests: value.requests,
        })
    }
    fn merge(&mut self, other: DeviceClaimConfigurationAc) -> Result<(), crate::Error> {
        if self.opaque.is_none() {
            self.opaque = crate::OptionableConvert::try_from_optioned(other.opaque)?;
        } else if let Some(self_value) = self.opaque.as_mut()
            && let Some(other_value) = other.opaque
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.requests.is_none() {
            self.requests = crate::OptionableConvert::try_from_optioned(other.requests)?;
        } else if let Some(self_value) = self.requests.as_mut()
            && let Some(other_value) = other.requests
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
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
