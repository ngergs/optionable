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
    /// If driver is set, only devices from that driver are selected. This fields corresponds to slice.spec.driver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<std::string::String>,
    /// If pool is set, only devices in that pool are selected.
    ///
    /// Also setting the driver name may be useful to avoid ambiguity when different drivers use the same pool name, but this is not required because selecting pools from different drivers may also be useful, for example when drivers with node-local devices use the node name as their pool name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<std::string::String>,
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
            driver: self.driver,
            pool: self.pool,
        }
    }
    fn try_from_optioned(value: DeviceTaintSelectorAc) -> Result<Self, crate::Error> {
        Ok(Self {
            device: value.device,
            driver: value.driver,
            pool: value.pool,
        })
    }
    fn merge(&mut self, other: DeviceTaintSelectorAc) -> Result<(), crate::Error> {
        self.device = other.device;
        self.driver = other.driver;
        self.pool = other.pool;
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
