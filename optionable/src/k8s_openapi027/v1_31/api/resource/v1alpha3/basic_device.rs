#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// BasicDevice defines one device instance.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BasicDeviceAc {
    /// Attributes defines the set of attributes for this device. The name of each attribute must be unique in that set.
    ///
    /// The maximum number of attributes and capacities combined is 32.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::api::resource::v1alpha3::DeviceAttribute as crate::Optionable>::Optioned,
        >,
    >,
    /// Capacity defines the set of capacities for this device. The name of each capacity must be unique in that set.
    ///
    /// The maximum number of attributes and capacities combined is 32.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1alpha3::BasicDevice {
    type Optioned = BasicDeviceAc;
}
#[automatically_derived]
impl crate::Optionable for BasicDeviceAc {
    type Optioned = BasicDeviceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::resource::v1alpha3::BasicDevice {
    fn into_optioned(self) -> BasicDeviceAc {
        BasicDeviceAc {
            attributes: crate::OptionableConvert::into_optioned(self.attributes),
            capacity: crate::OptionableConvert::into_optioned(self.capacity),
        }
    }
    fn try_from_optioned(value: BasicDeviceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            attributes: crate::OptionableConvert::try_from_optioned(value.attributes)?,
            capacity: crate::OptionableConvert::try_from_optioned(value.capacity)?,
        })
    }
    fn merge(&mut self, other: BasicDeviceAc) -> Result<(), crate::Error> {
        if self.attributes.is_none() {
            self.attributes = crate::OptionableConvert::try_from_optioned(
                other.attributes,
            )?;
        } else if let Some(self_value) = self.attributes.as_mut()
            && let Some(other_value) = other.attributes
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.capacity.is_none() {
            self.capacity = crate::OptionableConvert::try_from_optioned(other.capacity)?;
        } else if let Some(self_value) = self.capacity.as_mut()
            && let Some(other_value) = other.capacity
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1alpha3::BasicDevice>
for BasicDeviceAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::BasicDevice,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1alpha3::BasicDevice, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::BasicDevice,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for BasicDeviceAc {
    fn merge_from(&mut self, other: Self) {
        crate::k8s_openapi::merge::merge_granular_option_wrapped(
            &mut self.attributes,
            other.attributes,
        );
        crate::k8s_openapi::merge::merge_granular_option_wrapped(
            &mut self.capacity,
            other.capacity,
        );
    }
}
