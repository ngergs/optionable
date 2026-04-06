#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AllocatedDeviceStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Condition as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<
        <::k8s_openapi027::apimachinery::pkg::runtime::RawExtension as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_data: Option<
        <::k8s_openapi027::api::resource::v1::NetworkDeviceData as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<std::string::String>,
    #[serde(rename = "shareID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_id: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1::AllocatedDeviceStatus {
    type Optioned = AllocatedDeviceStatusAc;
}
#[automatically_derived]
impl crate::Optionable for AllocatedDeviceStatusAc {
    type Optioned = AllocatedDeviceStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1::AllocatedDeviceStatus {
    fn into_optioned(self) -> AllocatedDeviceStatusAc {
        AllocatedDeviceStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            data: crate::OptionableConvert::into_optioned(self.data),
            device: Some(self.device),
            driver: Some(self.driver),
            network_data: crate::OptionableConvert::into_optioned(self.network_data),
            pool: Some(self.pool),
            share_id: self.share_id,
        }
    }
    fn try_from_optioned(value: AllocatedDeviceStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            data: crate::OptionableConvert::try_from_optioned(value.data)?,
            device: value
                .device
                .ok_or(crate::Error {
                    missing_field: "device",
                })?,
            driver: value
                .driver
                .ok_or(crate::Error {
                    missing_field: "driver",
                })?,
            network_data: crate::OptionableConvert::try_from_optioned(
                value.network_data,
            )?,
            pool: value
                .pool
                .ok_or(crate::Error {
                    missing_field: "pool",
                })?,
            share_id: value.share_id,
        })
    }
    fn merge(&mut self, other: AllocatedDeviceStatusAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        crate::OptionableConvert::merge(&mut self.data, other.data)?;
        if let Some(other_value) = other.device {
            self.device = other_value;
        }
        if let Some(other_value) = other.driver {
            self.driver = other_value;
        }
        crate::OptionableConvert::merge(&mut self.network_data, other.network_data)?;
        if let Some(other_value) = other.pool {
            self.pool = other_value;
        }
        self.share_id = other.share_id;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1::AllocatedDeviceStatus>
for AllocatedDeviceStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1::AllocatedDeviceStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1::AllocatedDeviceStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1::AllocatedDeviceStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
