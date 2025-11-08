#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRequestAllocationResultAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_access: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerations: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1beta2::DeviceToleration>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1beta2::DeviceRequestAllocationResult {
    type Optioned = DeviceRequestAllocationResultAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceRequestAllocationResultAc {
    type Optioned = DeviceRequestAllocationResultAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta2::DeviceRequestAllocationResult {
    fn into_optioned(self) -> DeviceRequestAllocationResultAc {
        DeviceRequestAllocationResultAc {
            admin_access: crate::OptionableConvert::into_optioned(self.admin_access),
            device: Some(crate::OptionableConvert::into_optioned(self.device)),
            driver: Some(crate::OptionableConvert::into_optioned(self.driver)),
            pool: Some(crate::OptionableConvert::into_optioned(self.pool)),
            request: Some(crate::OptionableConvert::into_optioned(self.request)),
            tolerations: crate::OptionableConvert::into_optioned(self.tolerations),
        }
    }
    fn try_from_optioned(
        value: DeviceRequestAllocationResultAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            admin_access: crate::OptionableConvert::try_from_optioned(
                value.admin_access,
            )?,
            device: crate::OptionableConvert::try_from_optioned(
                value
                    .device
                    .ok_or(crate::Error {
                        missing_field: "device",
                    })?,
            )?,
            driver: crate::OptionableConvert::try_from_optioned(
                value
                    .driver
                    .ok_or(crate::Error {
                        missing_field: "driver",
                    })?,
            )?,
            pool: crate::OptionableConvert::try_from_optioned(
                value
                    .pool
                    .ok_or(crate::Error {
                        missing_field: "pool",
                    })?,
            )?,
            request: crate::OptionableConvert::try_from_optioned(
                value
                    .request
                    .ok_or(crate::Error {
                        missing_field: "request",
                    })?,
            )?,
            tolerations: crate::OptionableConvert::try_from_optioned(value.tolerations)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceRequestAllocationResultAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.admin_access, other.admin_access)?;
        if let Some(other_value) = other.device {
            crate::OptionableConvert::merge(&mut self.device, other_value)?;
        }
        if let Some(other_value) = other.driver {
            crate::OptionableConvert::merge(&mut self.driver, other_value)?;
        }
        if let Some(other_value) = other.pool {
            crate::OptionableConvert::merge(&mut self.pool, other_value)?;
        }
        if let Some(other_value) = other.request {
            crate::OptionableConvert::merge(&mut self.request, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.tolerations, other.tolerations)?;
        Ok(())
    }
}
