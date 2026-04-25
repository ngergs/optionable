#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DeviceCounterConsumption defines a set of counters that a device will consume from a CounterSet.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceCounterConsumptionAc {
    /// CounterSet is the name of the set from which the counters defined will be consumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counter_set: Option<std::string::String>,
    /// Counters defines the counters that will be consumed by the device.
    ///
    /// The maximum number of counters is 32.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counters: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::api::resource::v1beta2::Counter as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::resource::v1beta2::DeviceCounterConsumption {
    type Optioned = DeviceCounterConsumptionAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceCounterConsumptionAc {
    type Optioned = DeviceCounterConsumptionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1beta2::DeviceCounterConsumption {
    fn into_optioned(self) -> DeviceCounterConsumptionAc {
        DeviceCounterConsumptionAc {
            counter_set: Some(self.counter_set),
            counters: Some(crate::OptionableConvert::into_optioned(self.counters)),
        }
    }
    fn try_from_optioned(
        value: DeviceCounterConsumptionAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            counter_set: value
                .counter_set
                .ok_or(crate::Error {
                    missing_field: "counter_set",
                })?,
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
            self.counter_set = crate::OptionableConvert::try_from_optioned(other_value)?;
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
    k8s_openapi027::api::resource::v1beta2::DeviceCounterConsumption,
> for DeviceCounterConsumptionAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta2::DeviceCounterConsumption,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1beta2::DeviceCounterConsumption,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta2::DeviceCounterConsumption,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for DeviceCounterConsumptionAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.counter_set, other.counter_set);
        k8s_openapi027::DeepMerge::merge_from(&mut self.counters, other.counters);
    }
}
