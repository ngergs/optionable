pub struct AllocatedDeviceStatusOpt {
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>,
    > as crate::Optionable>::Optioned,
    pub data: <Option<
        ::k8s_openapi::apimachinery::pkg::runtime::RawExtension,
    > as crate::Optionable>::Optioned,
    pub device: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub driver: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub network_data: <Option<
        ::k8s_openapi::api::resource::v1beta1::NetworkDeviceData,
    > as crate::Optionable>::Optioned,
    pub pool: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub share_id: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta1::AllocatedDeviceStatus {
    type Optioned = AllocatedDeviceStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for AllocatedDeviceStatusOpt {
    type Optioned = AllocatedDeviceStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta1::AllocatedDeviceStatus {
    fn into_optioned(self) -> AllocatedDeviceStatusOpt {
        AllocatedDeviceStatusOpt {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            data: crate::OptionableConvert::into_optioned(self.data),
            device: Some(crate::OptionableConvert::into_optioned(self.device)),
            driver: Some(crate::OptionableConvert::into_optioned(self.driver)),
            network_data: crate::OptionableConvert::into_optioned(self.network_data),
            pool: Some(crate::OptionableConvert::into_optioned(self.pool)),
            share_id: crate::OptionableConvert::into_optioned(self.share_id),
        }
    }
    fn try_from_optioned(
        value: AllocatedDeviceStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            data: crate::OptionableConvert::try_from_optioned(value.data)?,
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
            network_data: crate::OptionableConvert::try_from_optioned(
                value.network_data,
            )?,
            pool: crate::OptionableConvert::try_from_optioned(
                value
                    .pool
                    .ok_or(crate::optionable::Error {
                        missing_field: "pool",
                    })?,
            )?,
            share_id: crate::OptionableConvert::try_from_optioned(value.share_id)?,
        })
    }
    fn merge(
        &mut self,
        other: AllocatedDeviceStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        crate::OptionableConvert::merge(&mut self.data, other.data)?;
        if let Some(other_value) = other.device {
            crate::OptionableConvert::merge(&mut self.device, other_value)?;
        }
        if let Some(other_value) = other.driver {
            crate::OptionableConvert::merge(&mut self.driver, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.network_data, other.network_data)?;
        if let Some(other_value) = other.pool {
            crate::OptionableConvert::merge(&mut self.pool, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.share_id, other.share_id)?;
        Ok(())
    }
}
