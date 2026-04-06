#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceRequestAllocationResultAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_access: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_conditions: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_failure_conditions: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_capacity: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<std::string::String>,
    #[serde(rename = "shareID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_id: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1::DeviceToleration as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::resource::v1::DeviceRequestAllocationResult {
    type Optioned = DeviceRequestAllocationResultAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceRequestAllocationResultAc {
    type Optioned = DeviceRequestAllocationResultAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1::DeviceRequestAllocationResult {
    fn into_optioned(self) -> DeviceRequestAllocationResultAc {
        DeviceRequestAllocationResultAc {
            admin_access: self.admin_access,
            binding_conditions: self.binding_conditions,
            binding_failure_conditions: self.binding_failure_conditions,
            consumed_capacity: crate::OptionableConvert::into_optioned(
                self.consumed_capacity,
            ),
            device: Some(self.device),
            driver: Some(self.driver),
            pool: Some(self.pool),
            request: Some(self.request),
            share_id: self.share_id,
            tolerations: crate::OptionableConvert::into_optioned(self.tolerations),
        }
    }
    fn try_from_optioned(
        value: DeviceRequestAllocationResultAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            admin_access: value.admin_access,
            binding_conditions: value.binding_conditions,
            binding_failure_conditions: value.binding_failure_conditions,
            consumed_capacity: crate::OptionableConvert::try_from_optioned(
                value.consumed_capacity,
            )?,
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
            pool: value
                .pool
                .ok_or(crate::Error {
                    missing_field: "pool",
                })?,
            request: value
                .request
                .ok_or(crate::Error {
                    missing_field: "request",
                })?,
            share_id: value.share_id,
            tolerations: crate::OptionableConvert::try_from_optioned(value.tolerations)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceRequestAllocationResultAc,
    ) -> Result<(), crate::Error> {
        self.admin_access = other.admin_access;
        self.binding_conditions = other.binding_conditions;
        self.binding_failure_conditions = other.binding_failure_conditions;
        crate::OptionableConvert::merge(
            &mut self.consumed_capacity,
            other.consumed_capacity,
        )?;
        if let Some(other_value) = other.device {
            self.device = other_value;
        }
        if let Some(other_value) = other.driver {
            self.driver = other_value;
        }
        if let Some(other_value) = other.pool {
            self.pool = other_value;
        }
        if let Some(other_value) = other.request {
            self.request = other_value;
        }
        self.share_id = other.share_id;
        crate::OptionableConvert::merge(&mut self.tolerations, other.tolerations)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::resource::v1::DeviceRequestAllocationResult,
> for DeviceRequestAllocationResultAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1::DeviceRequestAllocationResult,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1::DeviceRequestAllocationResult,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1::DeviceRequestAllocationResult,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
