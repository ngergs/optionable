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
    /// CompatibilityGroups is a list of opaque group names for this counter set consumption.
    ///
    /// Devices that consume counters from the same counter set may only be allocated at the same time ("co-allocated") if they all share at least one common group: the intersection of the CompatibilityGroups of all co-allocated devices on that counter set must be non-empty. Devices that consume from different counter sets are never compared via this field.
    ///
    /// An unset field, an explicit nil, and an empty list are equivalent and mean "no groups": such a device is only co-allocatable with sibling devices on the same counter set that also have no groups, and is never co-allocatable with a device that declares one or more groups.
    ///
    /// Group names are opaque and meaningful only within the publishing driver's pool.
    ///
    /// The maximum number of groups is 2, and the names must be unique.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_groups: Option<std::vec::Vec<std::string::String>>,
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
            <::k8s_openapi029::api::resource::v1beta2::Counter as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::resource::v1beta2::DeviceCounterConsumption {
    type Optioned = DeviceCounterConsumptionAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceCounterConsumptionAc {
    type Optioned = DeviceCounterConsumptionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::resource::v1beta2::DeviceCounterConsumption {
    fn into_optioned(self) -> DeviceCounterConsumptionAc {
        DeviceCounterConsumptionAc {
            compatibility_groups: self.compatibility_groups,
            counter_set: Some(self.counter_set),
            counters: Some(crate::OptionableConvert::into_optioned(self.counters)),
        }
    }
    fn try_from_optioned(
        value: DeviceCounterConsumptionAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            compatibility_groups: value.compatibility_groups,
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
        if self.compatibility_groups.is_none() {
            self.compatibility_groups = crate::OptionableConvert::try_from_optioned(
                other.compatibility_groups,
            )?;
        } else if let Some(self_value) = self.compatibility_groups.as_mut()
            && let Some(other_value) = other.compatibility_groups
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
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
    k8s_openapi029::api::resource::v1beta2::DeviceCounterConsumption,
> for DeviceCounterConsumptionAc {
    fn from_optionable(
        value: k8s_openapi029::api::resource::v1beta2::DeviceCounterConsumption,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::resource::v1beta2::DeviceCounterConsumption,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::resource::v1beta2::DeviceCounterConsumption,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for DeviceCounterConsumptionAc {
    fn merge_from(&mut self, other: Self) {
        self.compatibility_groups = other.compatibility_groups;
        k8s_openapi029::DeepMerge::merge_from(&mut self.counter_set, other.counter_set);
        crate::k8s_openapi::merge::merge_granular_option_wrapped(
            &mut self.counters,
            other.counters,
        );
    }
}
