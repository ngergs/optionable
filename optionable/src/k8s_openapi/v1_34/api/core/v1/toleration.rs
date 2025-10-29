pub struct TolerationOpt {
    pub effect: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub key: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub operator: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub toleration_seconds: <Option<i64> as crate::Optionable>::Optioned,
    pub value: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::Toleration {
    type Optioned = TolerationOpt;
}
#[automatically_derived]
impl crate::Optionable for TolerationOpt {
    type Optioned = TolerationOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Toleration {
    fn into_optioned(self) -> TolerationOpt {
        TolerationOpt {
            effect: crate::OptionableConvert::into_optioned(self.effect),
            key: crate::OptionableConvert::into_optioned(self.key),
            operator: crate::OptionableConvert::into_optioned(self.operator),
            toleration_seconds: crate::OptionableConvert::into_optioned(
                self.toleration_seconds,
            ),
            value: crate::OptionableConvert::into_optioned(self.value),
        }
    }
    fn try_from_optioned(
        value: TolerationOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            effect: crate::OptionableConvert::try_from_optioned(value.effect)?,
            key: crate::OptionableConvert::try_from_optioned(value.key)?,
            operator: crate::OptionableConvert::try_from_optioned(value.operator)?,
            toleration_seconds: crate::OptionableConvert::try_from_optioned(
                value.toleration_seconds,
            )?,
            value: crate::OptionableConvert::try_from_optioned(value.value)?,
        })
    }
    fn merge(&mut self, other: TolerationOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.effect, other.effect)?;
        crate::OptionableConvert::merge(&mut self.key, other.key)?;
        crate::OptionableConvert::merge(&mut self.operator, other.operator)?;
        crate::OptionableConvert::merge(
            &mut self.toleration_seconds,
            other.toleration_seconds,
        )?;
        crate::OptionableConvert::merge(&mut self.value, other.value)?;
        Ok(())
    }
}
