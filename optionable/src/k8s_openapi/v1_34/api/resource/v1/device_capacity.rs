#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceCapacityAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_policy: <Option<
        ::k8s_openapi::api::resource::v1::CapacityRequestPolicy,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<
        <::k8s_openapi::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::DeviceCapacity {
    type Optioned = DeviceCapacityAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceCapacityAc {
    type Optioned = DeviceCapacityAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1::DeviceCapacity {
    fn into_optioned(self) -> DeviceCapacityAc {
        DeviceCapacityAc {
            request_policy: crate::OptionableConvert::into_optioned(self.request_policy),
            value: Some(crate::OptionableConvert::into_optioned(self.value)),
        }
    }
    fn try_from_optioned(
        value: DeviceCapacityAc,
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
        other: DeviceCapacityAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.request_policy, other.request_policy)?;
        if let Some(other_value) = other.value {
            crate::OptionableConvert::merge(&mut self.value, other_value)?;
        }
        Ok(())
    }
}
