#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// AllocatedDeviceStatus contains the status of an allocated device, if the driver chooses to report it. This may include driver-specific information.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AllocatedDeviceStatusAc {
    /// Conditions contains the latest observation of the device's state. If the device has been configured according to the class and claim config references, the `Ready` condition should be True.
    ///
    /// Must not contain more than 8 entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Condition as crate::Optionable>::Optioned,
        >,
    >,
    /// Data contains arbitrary driver-specific data.
    ///
    /// The length of the raw data must be smaller or equal to 10 Ki.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<
        <::k8s_openapi027::apimachinery::pkg::runtime::RawExtension as crate::Optionable>::Optioned,
    >,
    /// Device references one device instance via its name in the driver's resource pool. It must be a DNS label.
    pub device: std::string::String,
    /// Driver specifies the name of the DRA driver whose kubelet plugin should be invoked to process the allocation once the claim is needed on a node.
    ///
    /// Must be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver.
    pub driver: std::string::String,
    /// NetworkData contains network-related information specific to the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_data: Option<
        <::k8s_openapi027::api::resource::v1beta1::NetworkDeviceData as crate::Optionable>::Optioned,
    >,
    /// This name together with the driver name and the device name field identify which device was allocated (`\<driver name\>/\<pool name\>/\<device name\>`).
    ///
    /// Must not be longer than 253 characters and may contain one or more DNS sub-domains separated by slashes.
    pub pool: std::string::String,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::resource::v1beta1::AllocatedDeviceStatus {
    type Optioned = AllocatedDeviceStatusAc;
}
#[automatically_derived]
impl crate::Optionable for AllocatedDeviceStatusAc {
    type Optioned = AllocatedDeviceStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1beta1::AllocatedDeviceStatus {
    fn into_optioned(self) -> AllocatedDeviceStatusAc {
        AllocatedDeviceStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            data: crate::OptionableConvert::into_optioned(self.data),
            device: self.device,
            driver: self.driver,
            network_data: crate::OptionableConvert::into_optioned(self.network_data),
            pool: self.pool,
        }
    }
    fn try_from_optioned(value: AllocatedDeviceStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            data: crate::OptionableConvert::try_from_optioned(value.data)?,
            device: value.device,
            driver: value.driver,
            network_data: crate::OptionableConvert::try_from_optioned(
                value.network_data,
            )?,
            pool: value.pool,
        })
    }
    fn merge(&mut self, other: AllocatedDeviceStatusAc) -> Result<(), crate::Error> {
        if self.conditions.is_none() {
            self.conditions = crate::OptionableConvert::try_from_optioned(
                other.conditions,
            )?;
        } else if let Some(self_value) = self.conditions.as_mut()
            && let Some(other_value) = other.conditions
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if self.data.is_none() {
            self.data = crate::OptionableConvert::try_from_optioned(other.data)?;
        } else if let Some(self_value) = self.data.as_mut()
            && let Some(other_value) = other.data
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        self.device = other.device;
        self.driver = other.driver;
        if self.network_data.is_none() {
            self.network_data = crate::OptionableConvert::try_from_optioned(
                other.network_data,
            )?;
        } else if let Some(self_value) = self.network_data.as_mut()
            && let Some(other_value) = other.network_data
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        self.pool = other.pool;
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi027::api::resource::v1beta1::AllocatedDeviceStatus {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.device == other.device && self.driver == other.driver
            && self.pool == other.pool
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::resource::v1beta1::AllocatedDeviceStatus,
> for AllocatedDeviceStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta1::AllocatedDeviceStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1beta1::AllocatedDeviceStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta1::AllocatedDeviceStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for AllocatedDeviceStatusAc {
    fn merge_from(&mut self, other: Self) {
        crate::k8s_openapi::merge::merge_map_option_wrapped(
            &mut self.conditions,
            other.conditions,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.data, other.data);
        k8s_openapi027::DeepMerge::merge_from(&mut self.device, other.device);
        k8s_openapi027::DeepMerge::merge_from(&mut self.driver, other.driver);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.network_data,
            other.network_data,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.pool, other.pool);
    }
}
impl crate::merge::MapKeysEq for AllocatedDeviceStatusAc {
    fn keys_eq(&self, other: &Self) -> bool {
        self.device == other.device && self.driver == other.driver
            && self.pool == other.pool
    }
}
