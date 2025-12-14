#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CounterSetAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counters: Option<
        <std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::api::resource::v1alpha3::Counter,
        > as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha3::CounterSet {
    type Optioned = CounterSetAc;
}
#[automatically_derived]
impl crate::Optionable for CounterSetAc {
    type Optioned = CounterSetAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1alpha3::CounterSet {
    fn into_optioned(self) -> CounterSetAc {
        CounterSetAc {
            counters: Some(crate::OptionableConvert::into_optioned(self.counters)),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
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
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: CounterSetAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.counters {
            crate::OptionableConvert::merge(&mut self.counters, other_value)?;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::resource::v1alpha3::CounterSet>
for CounterSetAc {
    fn from_optionable(
        value: ::k8s_openapi::api::resource::v1alpha3::CounterSet,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::resource::v1alpha3::CounterSet, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::resource::v1alpha3::CounterSet,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
