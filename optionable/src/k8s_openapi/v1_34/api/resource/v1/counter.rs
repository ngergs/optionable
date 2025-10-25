pub struct CounterOpt {
    pub value: Option<
        <::k8s_openapi::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::Counter {
    type Optioned = CounterOpt;
}
#[automatically_derived]
impl crate::Optionable for CounterOpt {
    type Optioned = CounterOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1::Counter {
    fn into_optioned(self) -> CounterOpt {
        CounterOpt {
            value: Some(
                <::k8s_openapi::apimachinery::pkg::api::resource::Quantity as crate::OptionableConvert>::into_optioned(
                    self.value,
                ),
            ),
        }
    }
    fn try_from_optioned(value: CounterOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            value: <::k8s_openapi::apimachinery::pkg::api::resource::Quantity as crate::OptionableConvert>::try_from_optioned(
                value
                    .value
                    .ok_or(crate::optionable::Error {
                        missing_field: "value",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: CounterOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.value {
            <::k8s_openapi::apimachinery::pkg::api::resource::Quantity as crate::OptionableConvert>::merge(
                &mut self.value,
                other_value,
            )?;
        }
        Ok(())
    }
}
