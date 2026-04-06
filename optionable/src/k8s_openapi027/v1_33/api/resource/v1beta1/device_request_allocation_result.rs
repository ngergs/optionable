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
    pub device: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1beta1::DeviceToleration as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::resource::v1beta1::DeviceRequestAllocationResult {
    type Optioned = DeviceRequestAllocationResultAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceRequestAllocationResultAc {
    type Optioned = DeviceRequestAllocationResultAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1beta1::DeviceRequestAllocationResult {
    fn into_optioned(self) -> DeviceRequestAllocationResultAc {
        DeviceRequestAllocationResultAc {
            admin_access: self.admin_access,
            device: Some(self.device),
            driver: Some(self.driver),
            pool: Some(self.pool),
            request: Some(self.request),
            tolerations: crate::OptionableConvert::into_optioned(self.tolerations),
        }
    }
    fn try_from_optioned(
        value: DeviceRequestAllocationResultAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            admin_access: value.admin_access,
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
            tolerations: crate::OptionableConvert::try_from_optioned(value.tolerations)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceRequestAllocationResultAc,
    ) -> Result<(), crate::Error> {
        self.admin_access = other.admin_access;
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
        crate::OptionableConvert::merge(&mut self.tolerations, other.tolerations)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::resource::v1beta1::DeviceRequestAllocationResult,
> for DeviceRequestAllocationResultAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta1::DeviceRequestAllocationResult,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1beta1::DeviceRequestAllocationResult,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta1::DeviceRequestAllocationResult,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
