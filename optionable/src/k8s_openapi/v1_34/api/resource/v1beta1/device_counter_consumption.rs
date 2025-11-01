pub struct DeviceCounterConsumptionAc {
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
for ::k8s_openapi::api::resource::v1beta1::DeviceCounterConsumption {
    type Optioned = DeviceCounterConsumptionAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceCounterConsumptionAc {
    type Optioned = DeviceCounterConsumptionAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta1::DeviceCounterConsumption {
    fn into_optioned(self) -> DeviceCounterConsumptionAc {
        DeviceCounterConsumptionAc {
            counter_set: Some(crate::OptionableConvert::into_optioned(self.counter_set)),
            counters: Some(crate::OptionableConvert::into_optioned(self.counters)),
        }
    }
    fn try_from_optioned(
        value: DeviceCounterConsumptionAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            counter_set: crate::OptionableConvert::try_from_optioned(
                value
                    .counter_set
                    .ok_or(crate::optionable::Error {
                        missing_field: "counter_set",
                    })?,
            )?,
            counters: crate::OptionableConvert::try_from_optioned(
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
        other: DeviceCounterConsumptionAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.counter_set {
            crate::OptionableConvert::merge(&mut self.counter_set, other_value)?;
        }
        if let Some(other_value) = other.counters {
            crate::OptionableConvert::merge(&mut self.counters, other_value)?;
        }
        Ok(())
    }
}
