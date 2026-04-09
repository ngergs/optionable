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
        <::k8s_openapi027::api::resource::v1beta2::NetworkDeviceData as crate::Optionable>::Optioned,
    >,
    /// This name together with the driver name and the device name field identify which device was allocated (`\<driver name\>/\<pool name\>/\<device name\>`).
    ///
    /// Must not be longer than 253 characters and may contain one or more DNS sub-domains separated by slashes.
    pub pool: std::string::String,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::resource::v1beta2::AllocatedDeviceStatus {
    type Optioned = AllocatedDeviceStatusAc;
}
#[automatically_derived]
impl crate::Optionable for AllocatedDeviceStatusAc {
    type Optioned = AllocatedDeviceStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1beta2::AllocatedDeviceStatus {
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
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        crate::OptionableConvert::merge(&mut self.data, other.data)?;
        self.device = other.device;
        self.driver = other.driver;
        crate::OptionableConvert::merge(&mut self.network_data, other.network_data)?;
        self.pool = other.pool;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::resource::v1beta2::AllocatedDeviceStatus,
> for AllocatedDeviceStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta2::AllocatedDeviceStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1beta2::AllocatedDeviceStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta2::AllocatedDeviceStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
