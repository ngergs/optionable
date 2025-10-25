pub struct QuantityOpt(pub Option<<std::string::String as crate::Optionable>::Optioned>);
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::api::resource::quantity::Quantity {
    type Optioned = QuantityOpt;
}
#[automatically_derived]
impl crate::Optionable for QuantityOpt {
    type Optioned = QuantityOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::api::resource::quantity::Quantity {
    fn into_optioned(self) -> QuantityOpt {
        QuantityOpt(
            Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(self.0),
            ),
        )
    }
    fn try_from_optioned(value: QuantityOpt) -> Result<Self, crate::optionable::Error> {
        Ok(
            Self(
                <std::string::String as crate::OptionableConvert>::try_from_optioned(
                    value
                        .0
                        .ok_or(crate::optionable::Error {
                            missing_field: "0",
                        })?,
                )?,
            ),
        )
    }
    fn merge(&mut self, other: QuantityOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.0 {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.0,
                other_value,
            )?;
        }
        Ok(())
    }
}
