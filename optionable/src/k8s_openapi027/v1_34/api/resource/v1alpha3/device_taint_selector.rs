#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceTaintSelectorAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class_name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<std::string::String>,
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
        self.device = other.device;
        self.device_class_name = other.device_class_name;
        self.driver = other.driver;
        self.pool = other.pool;
        crate::OptionableConvert::merge(&mut self.selectors, other.selectors)?;
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
