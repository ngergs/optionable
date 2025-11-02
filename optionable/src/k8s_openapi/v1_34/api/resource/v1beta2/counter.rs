#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CounterAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<
        <::k8s_openapi::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta2::Counter {
    type Optioned = CounterAc;
}
#[automatically_derived]
impl crate::Optionable for CounterAc {
    type Optioned = CounterAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1beta2::Counter {
    fn into_optioned(self) -> CounterAc {
        CounterAc {
            value: Some(crate::OptionableConvert::into_optioned(self.value)),
        }
    }
    fn try_from_optioned(value: CounterAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            value: crate::OptionableConvert::try_from_optioned(
                value
                    .value
                    .ok_or(crate::optionable::Error {
                        missing_field: "value",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: CounterAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.value {
            crate::OptionableConvert::merge(&mut self.value, other_value)?;
        }
        Ok(())
    }
}
