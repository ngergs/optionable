#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceCounterConsumptionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counter_set: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counters: Option<
        <std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::api::resource::v1beta2::Counter,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1beta2::DeviceCounterConsumption {
    type Optioned = DeviceCounterConsumptionAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceCounterConsumptionAc {
    type Optioned = DeviceCounterConsumptionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta2::DeviceCounterConsumption {
    fn into_optioned(self) -> DeviceCounterConsumptionAc {
        DeviceCounterConsumptionAc {
            counter_set: Some(crate::OptionableConvert::into_optioned(self.counter_set)),
            counters: Some(crate::OptionableConvert::into_optioned(self.counters)),
        }
    }
    fn try_from_optioned(
        value: DeviceCounterConsumptionAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            counter_set: crate::OptionableConvert::try_from_optioned(
                value
                    .counter_set
                    .ok_or(crate::Error {
                        missing_field: "counter_set",
                    })?,
            )?,
            counters: crate::OptionableConvert::try_from_optioned(
                value
                    .counters
                    .ok_or(crate::Error {
                        missing_field: "counters",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: DeviceCounterConsumptionAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.counter_set {
            crate::OptionableConvert::merge(&mut self.counter_set, other_value)?;
        }
        if let Some(other_value) = other.counters {
            crate::OptionableConvert::merge(&mut self.counters, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::api::resource::v1beta2::DeviceCounterConsumption,
> for DeviceCounterConsumptionAc {
    fn from_optionable(
        value: ::k8s_openapi::api::resource::v1beta2::DeviceCounterConsumption,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::resource::v1beta2::DeviceCounterConsumption,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::resource::v1beta2::DeviceCounterConsumption,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
