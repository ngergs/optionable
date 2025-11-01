pub struct TaintAc {
    pub effect: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub key: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub time_added: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    pub value: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::Taint {
    type Optioned = TaintAc;
}
#[automatically_derived]
impl crate::Optionable for TaintAc {
    type Optioned = TaintAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Taint {
    fn into_optioned(self) -> TaintAc {
        TaintAc {
            effect: Some(crate::OptionableConvert::into_optioned(self.effect)),
            key: Some(crate::OptionableConvert::into_optioned(self.key)),
            time_added: crate::OptionableConvert::into_optioned(self.time_added),
            value: crate::OptionableConvert::into_optioned(self.value),
        }
    }
    fn try_from_optioned(value: TaintAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            effect: crate::OptionableConvert::try_from_optioned(
                value
                    .effect
                    .ok_or(crate::optionable::Error {
                        missing_field: "effect",
                    })?,
            )?,
            key: crate::OptionableConvert::try_from_optioned(
                value
                    .key
                    .ok_or(crate::optionable::Error {
                        missing_field: "key",
                    })?,
            )?,
            time_added: crate::OptionableConvert::try_from_optioned(value.time_added)?,
            value: crate::OptionableConvert::try_from_optioned(value.value)?,
        })
    }
    fn merge(&mut self, other: TaintAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.effect {
            crate::OptionableConvert::merge(&mut self.effect, other_value)?;
        }
        if let Some(other_value) = other.key {
            crate::OptionableConvert::merge(&mut self.key, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.time_added, other.time_added)?;
        crate::OptionableConvert::merge(&mut self.value, other.value)?;
        Ok(())
    }
}
