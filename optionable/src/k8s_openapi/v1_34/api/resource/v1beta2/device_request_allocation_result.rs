pub struct DeviceRequestAllocationResultAc {
    pub admin_access: <Option<bool> as crate::Optionable>::Optioned,
    pub binding_conditions: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub binding_failure_conditions: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub consumed_capacity: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    pub device: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub driver: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub pool: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub request: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub share_id: <Option<std::string::String> as crate::Optionable>::Optioned,
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
            binding_conditions: crate::OptionableConvert::into_optioned(
                self.binding_conditions,
            ),
            binding_failure_conditions: crate::OptionableConvert::into_optioned(
                self.binding_failure_conditions,
            ),
            consumed_capacity: crate::OptionableConvert::into_optioned(
                self.consumed_capacity,
            ),
            device: Some(crate::OptionableConvert::into_optioned(self.device)),
            driver: Some(crate::OptionableConvert::into_optioned(self.driver)),
            pool: Some(crate::OptionableConvert::into_optioned(self.pool)),
            request: Some(crate::OptionableConvert::into_optioned(self.request)),
            share_id: crate::OptionableConvert::into_optioned(self.share_id),
            tolerations: crate::OptionableConvert::into_optioned(self.tolerations),
        }
    }
    fn try_from_optioned(
        value: DeviceRequestAllocationResultAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            admin_access: crate::OptionableConvert::try_from_optioned(
                value.admin_access,
            )?,
            binding_conditions: crate::OptionableConvert::try_from_optioned(
                value.binding_conditions,
            )?,
            binding_failure_conditions: crate::OptionableConvert::try_from_optioned(
                value.binding_failure_conditions,
            )?,
            consumed_capacity: crate::OptionableConvert::try_from_optioned(
                value.consumed_capacity,
            )?,
            device: crate::OptionableConvert::try_from_optioned(
                value
                    .device
                    .ok_or(crate::optionable::Error {
                        missing_field: "device",
                    })?,
            )?,
            driver: crate::OptionableConvert::try_from_optioned(
                value
                    .driver
                    .ok_or(crate::optionable::Error {
                        missing_field: "driver",
                    })?,
            )?,
            pool: crate::OptionableConvert::try_from_optioned(
                value
                    .pool
                    .ok_or(crate::optionable::Error {
                        missing_field: "pool",
                    })?,
            )?,
            request: crate::OptionableConvert::try_from_optioned(
                value
                    .request
                    .ok_or(crate::optionable::Error {
                        missing_field: "request",
                    })?,
            )?,
            share_id: crate::OptionableConvert::try_from_optioned(value.share_id)?,
            tolerations: crate::OptionableConvert::try_from_optioned(value.tolerations)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceRequestAllocationResultAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.admin_access, other.admin_access)?;
        crate::OptionableConvert::merge(
            &mut self.binding_conditions,
            other.binding_conditions,
        )?;
        crate::OptionableConvert::merge(
            &mut self.binding_failure_conditions,
            other.binding_failure_conditions,
        )?;
        crate::OptionableConvert::merge(
            &mut self.consumed_capacity,
            other.consumed_capacity,
        )?;
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
        crate::OptionableConvert::merge(&mut self.share_id, other.share_id)?;
        crate::OptionableConvert::merge(&mut self.tolerations, other.tolerations)?;
        Ok(())
    }
}
