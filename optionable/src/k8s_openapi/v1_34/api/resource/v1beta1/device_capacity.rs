pub struct DeviceCapacityOpt {
    pub request_policy: <Option<
        ::k8s_openapi::api::resource::v1beta1::CapacityRequestPolicy,
    > as crate::Optionable>::Optioned,
    pub value: Option<
        <::k8s_openapi::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta1::DeviceCapacity {
    type Optioned = DeviceCapacityOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceCapacityOpt {
    type Optioned = DeviceCapacityOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1beta1::DeviceCapacity {
    fn into_optioned(self) -> DeviceCapacityOpt {
        DeviceCapacityOpt {
            request_policy: crate::OptionableConvert::into_optioned(self.request_policy),
            value: Some(crate::OptionableConvert::into_optioned(self.value)),
        }
    }
    fn try_from_optioned(
        value: DeviceCapacityOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            request_policy: crate::OptionableConvert::try_from_optioned(
                value.request_policy,
            )?,
            value: crate::OptionableConvert::try_from_optioned(
                value
                    .value
                    .ok_or(crate::optionable::Error {
                        missing_field: "value",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceCapacityOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.request_policy, other.request_policy)?;
        if let Some(other_value) = other.value {
            crate::OptionableConvert::merge(&mut self.value, other_value)?;
        }
        Ok(())
    }
}
