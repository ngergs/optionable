#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct QuantityAc(
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Option<<std::string::String as crate::Optionable>::Optioned>,
);
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::apimachinery::pkg::api::resource::Quantity {
    type Optioned = QuantityAc;
}
#[automatically_derived]
impl crate::Optionable for QuantityAc {
    type Optioned = QuantityAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::api::resource::Quantity {
    fn into_optioned(self) -> QuantityAc {
        QuantityAc(Some(crate::OptionableConvert::into_optioned(self.0)))
    }
    fn try_from_optioned(value: QuantityAc) -> Result<Self, crate::optionable::Error> {
        Ok(
            Self(
                crate::OptionableConvert::try_from_optioned(
                    value
                        .0
                        .ok_or(crate::optionable::Error {
                            missing_field: "0",
                        })?,
                )?,
            ),
        )
    }
    fn merge(&mut self, other: QuantityAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.0 {
            crate::OptionableConvert::merge(&mut self.0, other_value)?;
        }
        Ok(())
    }
}
