pub struct DeviceCounterConsumptionOpt {
    pub counter_set: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub counters: Option<
        <std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::api::resource::v1beta1::Counter,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1beta1::device_counter_consumption::DeviceCounterConsumption {
    type Optioned = DeviceCounterConsumptionOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceCounterConsumptionOpt {
    type Optioned = DeviceCounterConsumptionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta1::device_counter_consumption::DeviceCounterConsumption {
    fn into_optioned(self) -> DeviceCounterConsumptionOpt {
        DeviceCounterConsumptionOpt {
            counter_set: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.counter_set,
                ),
            ),
            counters: Some(
                <std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::api::resource::v1beta1::Counter,
                > as crate::OptionableConvert>::into_optioned(self.counters),
            ),
        }
    }
    fn try_from_optioned(
        value: DeviceCounterConsumptionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            counter_set: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .counter_set
                    .ok_or(crate::optionable::Error {
                        missing_field: "counter_set",
                    })?,
            )?,
            counters: <std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::api::resource::v1beta1::Counter,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .counters
                    .ok_or(crate::optionable::Error {
                        missing_field: "counters",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceCounterConsumptionOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.counter_set {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.counter_set,
                other_value,
            )?;
        }
        if let Some(other_value) = other.counters {
            <std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::api::resource::v1beta1::Counter,
            > as crate::OptionableConvert>::merge(&mut self.counters, other_value)?;
        }
        Ok(())
    }
}
