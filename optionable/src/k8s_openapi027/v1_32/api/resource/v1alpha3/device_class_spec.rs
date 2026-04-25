#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DeviceClassSpec is used in a \[DeviceClass\] to define what can be allocated and how to configure it.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceClassSpecAc {
    /// Config defines configuration parameters that apply to each device that is claimed via this class. Some classses may potentially be satisfied by multiple drivers, so each instance of a vendor configuration applies to exactly one driver.
    ///
    /// They are passed to the driver, but are not considered while allocating the claim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1alpha3::DeviceClassConfiguration as crate::Optionable>::Optioned,
        >,
    >,
    /// Each selector must be satisfied by a device which is claimed via this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1alpha3::DeviceSelector as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1alpha3::DeviceClassSpec {
    type Optioned = DeviceClassSpecAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceClassSpecAc {
    type Optioned = DeviceClassSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1alpha3::DeviceClassSpec {
    fn into_optioned(self) -> DeviceClassSpecAc {
        DeviceClassSpecAc {
            config: crate::OptionableConvert::into_optioned(self.config),
            selectors: crate::OptionableConvert::into_optioned(self.selectors),
        }
    }
    fn try_from_optioned(value: DeviceClassSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            config: crate::OptionableConvert::try_from_optioned(value.config)?,
            selectors: crate::OptionableConvert::try_from_optioned(value.selectors)?,
        })
    }
    fn merge(&mut self, other: DeviceClassSpecAc) -> Result<(), crate::Error> {
        if self.config.is_none() {
            self.config = crate::OptionableConvert::try_from_optioned(other.config)?;
        } else if let Some(self_value) = self.config.as_mut()
            && let Some(other_value) = other.config
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.selectors.is_none() {
            self.selectors = crate::OptionableConvert::try_from_optioned(
                other.selectors,
            )?;
        } else if let Some(self_value) = self.selectors.as_mut()
            && let Some(other_value) = other.selectors
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1alpha3::DeviceClassSpec>
for DeviceClassSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::DeviceClassSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1alpha3::DeviceClassSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::DeviceClassSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for DeviceClassSpecAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.config, other.config);
        k8s_openapi027::DeepMerge::merge_from(&mut self.selectors, other.selectors);
    }
}
