pub struct DeviceRequestAllocationResultOpt {
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
        std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceToleration>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1::DeviceRequestAllocationResult {
    type Optioned = DeviceRequestAllocationResultOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceRequestAllocationResultOpt {
    type Optioned = DeviceRequestAllocationResultOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1::DeviceRequestAllocationResult {
    fn into_optioned(self) -> DeviceRequestAllocationResultOpt {
        DeviceRequestAllocationResultOpt {
            admin_access: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.admin_access),
            binding_conditions: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.binding_conditions),
            binding_failure_conditions: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(
                self.binding_failure_conditions,
            ),
            consumed_capacity: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::into_optioned(self.consumed_capacity),
            device: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.device,
                ),
            ),
            driver: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.driver,
                ),
            ),
            pool: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.pool,
                ),
            ),
            request: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.request,
                ),
            ),
            share_id: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.share_id),
            tolerations: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceToleration>,
            > as crate::OptionableConvert>::into_optioned(self.tolerations),
        }
    }
    fn try_from_optioned(
        value: DeviceRequestAllocationResultOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            admin_access: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.admin_access)?,
            binding_conditions: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.binding_conditions)?,
            binding_failure_conditions: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(
                value.binding_failure_conditions,
            )?,
            consumed_capacity: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.consumed_capacity)?,
            device: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .device
                    .ok_or(crate::optionable::Error {
                        missing_field: "device",
                    })?,
            )?,
            driver: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .driver
                    .ok_or(crate::optionable::Error {
                        missing_field: "driver",
                    })?,
            )?,
            pool: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .pool
                    .ok_or(crate::optionable::Error {
                        missing_field: "pool",
                    })?,
            )?,
            request: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .request
                    .ok_or(crate::optionable::Error {
                        missing_field: "request",
                    })?,
            )?,
            share_id: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.share_id)?,
            tolerations: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceToleration>,
            > as crate::OptionableConvert>::try_from_optioned(value.tolerations)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceRequestAllocationResultOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.admin_access,
            other.admin_access,
        )?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.binding_conditions,
            other.binding_conditions,
        )?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.binding_failure_conditions,
            other.binding_failure_conditions,
        )?;
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.consumed_capacity,
            other.consumed_capacity,
        )?;
        if let Some(other_value) = other.device {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.device,
                other_value,
            )?;
        }
        if let Some(other_value) = other.driver {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.driver,
                other_value,
            )?;
        }
        if let Some(other_value) = other.pool {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.pool,
                other_value,
            )?;
        }
        if let Some(other_value) = other.request {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.request,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.share_id, other.share_id)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceToleration>,
        > as crate::OptionableConvert>::merge(&mut self.tolerations, other.tolerations)?;
        Ok(())
    }
}
