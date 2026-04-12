#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DeviceTaintSelector defines which device(s) a DeviceTaintRule applies to. The empty selector matches all devices. Without a selector, no devices are matched.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceTaintSelectorAc {
    /// If device is set, only devices with that name are selected. This field corresponds to slice.spec.devices\[\].name.
    ///
    /// Setting also driver and pool may be required to avoid ambiguity, but is not required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<std::string::String>,
    /// If DeviceClassName is set, the selectors defined there must be satisfied by a device to be selected. This field corresponds to class.metadata.name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class_name: Option<std::string::String>,
    /// If driver is set, only devices from that driver are selected. This fields corresponds to slice.spec.driver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<std::string::String>,
    /// If pool is set, only devices in that pool are selected.
    ///
    /// Also setting the driver name may be useful to avoid ambiguity when different drivers use the same pool name, but this is not required because selecting pools from different drivers may also be useful, for example when drivers with node-local devices use the node name as their pool name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<std::string::String>,
    /// Selectors contains the same selection criteria as a ResourceClaim. Currently, CEL expressions are supported. All of these selectors must be satisfied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1alpha3::DeviceSelector as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1alpha3::DeviceTaintSelector {
    type Optioned = DeviceTaintSelectorAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceTaintSelectorAc {
    type Optioned = DeviceTaintSelectorAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1alpha3::DeviceTaintSelector {
    fn into_optioned(self) -> DeviceTaintSelectorAc {
        DeviceTaintSelectorAc {
            device: self.device,
            device_class_name: self.device_class_name,
            driver: self.driver,
            pool: self.pool,
            selectors: crate::OptionableConvert::into_optioned(self.selectors),
        }
    }
    fn try_from_optioned(value: DeviceTaintSelectorAc) -> Result<Self, crate::Error> {
        Ok(Self {
            device: value.device,
            device_class_name: value.device_class_name,
            driver: value.driver,
            pool: value.pool,
            selectors: crate::OptionableConvert::try_from_optioned(value.selectors)?,
        })
    }
    fn merge(&mut self, other: DeviceTaintSelectorAc) -> Result<(), crate::Error> {
        if self.device.is_none() {
            self.device = crate::OptionableConvert::try_from_optioned(other.device)?;
        } else if let Some(self_value) = self.device.as_mut()
            && let Some(other_value) = other.device
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.device_class_name.is_none() {
            self.device_class_name = crate::OptionableConvert::try_from_optioned(
                other.device_class_name,
            )?;
        } else if let Some(self_value) = self.device_class_name.as_mut()
            && let Some(other_value) = other.device_class_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.driver.is_none() {
            self.driver = crate::OptionableConvert::try_from_optioned(other.driver)?;
        } else if let Some(self_value) = self.driver.as_mut()
            && let Some(other_value) = other.driver
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.pool.is_none() {
            self.pool = crate::OptionableConvert::try_from_optioned(other.pool)?;
        } else if let Some(self_value) = self.pool.as_mut()
            && let Some(other_value) = other.pool
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
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1alpha3::DeviceTaintSelector>
for DeviceTaintSelectorAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::DeviceTaintSelector,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1alpha3::DeviceTaintSelector,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::DeviceTaintSelector,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
