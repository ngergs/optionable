pub struct CounterSetOpt {
    pub counters: Option<
        <std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::api::resource::v1beta2::Counter,
        > as crate::Optionable>::Optioned,
    >,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta2::CounterSet {
    type Optioned = CounterSetOpt;
}
#[automatically_derived]
impl crate::Optionable for CounterSetOpt {
    type Optioned = CounterSetOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1beta2::CounterSet {
    fn into_optioned(self) -> CounterSetOpt {
        CounterSetOpt {
            counters: Some(crate::OptionableConvert::into_optioned(self.counters)),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(
        value: CounterSetOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            counters: crate::OptionableConvert::try_from_optioned(
                value
                    .counters
                    .ok_or(crate::optionable::Error {
                        missing_field: "counters",
                    })?,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: CounterSetOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.counters {
            crate::OptionableConvert::merge(&mut self.counters, other_value)?;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}
