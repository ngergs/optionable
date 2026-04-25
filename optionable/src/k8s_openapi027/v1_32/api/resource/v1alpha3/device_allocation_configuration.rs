#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DeviceAllocationConfiguration gets embedded in an AllocationResult.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceAllocationConfigurationAc {
    /// Opaque provides driver-specific configuration parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opaque: Option<
        <::k8s_openapi027::api::resource::v1alpha3::OpaqueDeviceConfiguration as crate::Optionable>::Optioned,
    >,
    /// Requests lists the names of requests where the configuration applies. If empty, its applies to all requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<std::vec::Vec<std::string::String>>,
    /// Source records whether the configuration comes from a class and thus is not something that a normal user would have been able to set or from a claim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::resource::v1alpha3::DeviceAllocationConfiguration {
    type Optioned = DeviceAllocationConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceAllocationConfigurationAc {
    type Optioned = DeviceAllocationConfigurationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1alpha3::DeviceAllocationConfiguration {
    fn into_optioned(self) -> DeviceAllocationConfigurationAc {
        DeviceAllocationConfigurationAc {
            opaque: crate::OptionableConvert::into_optioned(self.opaque),
            requests: self.requests,
            source: Some(self.source),
        }
    }
    fn try_from_optioned(
        value: DeviceAllocationConfigurationAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            opaque: crate::OptionableConvert::try_from_optioned(value.opaque)?,
            requests: value.requests,
            source: value
                .source
                .ok_or(crate::Error {
                    missing_field: "source",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceAllocationConfigurationAc,
    ) -> Result<(), crate::Error> {
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
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.source {
            self.source = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::resource::v1alpha3::DeviceAllocationConfiguration,
> for DeviceAllocationConfigurationAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::DeviceAllocationConfiguration,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1alpha3::DeviceAllocationConfiguration,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::DeviceAllocationConfiguration,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for DeviceAllocationConfigurationAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.opaque, other.opaque);
        k8s_openapi027::DeepMerge::merge_from(&mut self.requests, other.requests);
        k8s_openapi027::DeepMerge::merge_from(&mut self.source, other.source);
    }
}
