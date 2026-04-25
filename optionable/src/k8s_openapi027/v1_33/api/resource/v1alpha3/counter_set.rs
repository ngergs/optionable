#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CounterSet defines a named set of counters that are available to be used by devices defined in the ResourceSlice.
///
/// The counters are not allocatable by themselves, but can be referenced by devices. When a device is allocated, the portion of counters it uses will no longer be available for use by other devices.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CounterSetAc {
    /// Counters defines the counters that will be consumed by the device. The name of each counter must be unique in that set and must be a DNS label.
    ///
    /// To ensure this uniqueness, capacities defined by the vendor must be listed without the driver name as domain prefix in their name. All others must be listed with their domain prefix.
    ///
    /// The maximum number of counters is 32.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counters: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::api::resource::v1alpha3::Counter as crate::Optionable>::Optioned,
        >,
    >,
    /// CounterSet is the name of the set from which the counters defined will be consumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1alpha3::CounterSet {
    type Optioned = CounterSetAc;
}
#[automatically_derived]
impl crate::Optionable for CounterSetAc {
    type Optioned = CounterSetAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::resource::v1alpha3::CounterSet {
    fn into_optioned(self) -> CounterSetAc {
        CounterSetAc {
            counters: Some(crate::OptionableConvert::into_optioned(self.counters)),
            name: Some(self.name),
        }
    }
    fn try_from_optioned(value: CounterSetAc) -> Result<Self, crate::Error> {
        Ok(Self {
            counters: crate::OptionableConvert::try_from_optioned(
                value
                    .counters
                    .ok_or(crate::Error {
                        missing_field: "counters",
                    })?,
            )?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
        })
    }
    fn merge(&mut self, other: CounterSetAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.counters {
            crate::OptionableConvert::merge(&mut self.counters, other_value)?;
        }
        if let Some(other_value) = other.name {
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1alpha3::CounterSet>
for CounterSetAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::CounterSet,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1alpha3::CounterSet, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::CounterSet,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for CounterSetAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.counters, other.counters);
        k8s_openapi027::DeepMerge::merge_from(&mut self.name, other.name);
    }
}
