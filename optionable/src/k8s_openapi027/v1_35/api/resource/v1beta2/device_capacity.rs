#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceCapacityAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_policy: <Option<
        ::k8s_openapi027::api::resource::v1beta2::CapacityRequestPolicy,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<
        <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1beta2::DeviceCapacity {
    type Optioned = DeviceCapacityAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceCapacityAc {
    type Optioned = DeviceCapacityAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1beta2::DeviceCapacity {
    fn into_optioned(self) -> DeviceCapacityAc {
        DeviceCapacityAc {
            request_policy: crate::OptionableConvert::into_optioned(self.request_policy),
            value: Some(crate::OptionableConvert::into_optioned(self.value)),
        }
    }
    fn try_from_optioned(value: DeviceCapacityAc) -> Result<Self, crate::Error> {
        Ok(Self {
            request_policy: crate::OptionableConvert::try_from_optioned(
                value.request_policy,
            )?,
            value: crate::OptionableConvert::try_from_optioned(
                value
                    .value
                    .ok_or(crate::Error {
                        missing_field: "value",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: DeviceCapacityAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.request_policy, other.request_policy)?;
        if let Some(other_value) = other.value {
            crate::OptionableConvert::merge(&mut self.value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1beta2::DeviceCapacity>
for DeviceCapacityAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta2::DeviceCapacity,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1beta2::DeviceCapacity, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta2::DeviceCapacity,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
