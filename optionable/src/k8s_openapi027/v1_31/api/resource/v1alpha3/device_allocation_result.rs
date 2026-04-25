#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DeviceAllocationResult is the result of allocating devices.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceAllocationResultAc {
    /// This field is a combination of all the claim and class configuration parameters. Drivers can distinguish between those based on a flag.
    ///
    /// This includes configuration parameters for drivers which have no allocated devices in the result because it is up to the drivers which configuration parameters they support. They can silently ignore unknown configuration parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1alpha3::DeviceAllocationConfiguration as crate::Optionable>::Optioned,
        >,
    >,
    /// Results lists all allocated devices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1alpha3::DeviceRequestAllocationResult as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::resource::v1alpha3::DeviceAllocationResult {
    type Optioned = DeviceAllocationResultAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceAllocationResultAc {
    type Optioned = DeviceAllocationResultAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1alpha3::DeviceAllocationResult {
    fn into_optioned(self) -> DeviceAllocationResultAc {
        DeviceAllocationResultAc {
            config: crate::OptionableConvert::into_optioned(self.config),
            results: crate::OptionableConvert::into_optioned(self.results),
        }
    }
    fn try_from_optioned(value: DeviceAllocationResultAc) -> Result<Self, crate::Error> {
        Ok(Self {
            config: crate::OptionableConvert::try_from_optioned(value.config)?,
            results: crate::OptionableConvert::try_from_optioned(value.results)?,
        })
    }
    fn merge(&mut self, other: DeviceAllocationResultAc) -> Result<(), crate::Error> {
        if self.config.is_none() {
            self.config = crate::OptionableConvert::try_from_optioned(other.config)?;
        } else if let Some(self_value) = self.config.as_mut()
            && let Some(other_value) = other.config
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.results.is_none() {
            self.results = crate::OptionableConvert::try_from_optioned(other.results)?;
        } else if let Some(self_value) = self.results.as_mut()
            && let Some(other_value) = other.results
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::resource::v1alpha3::DeviceAllocationResult,
> for DeviceAllocationResultAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::DeviceAllocationResult,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1alpha3::DeviceAllocationResult,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::DeviceAllocationResult,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for DeviceAllocationResultAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.config, other.config);
        k8s_openapi027::DeepMerge::merge_from(&mut self.results, other.results);
    }
}
