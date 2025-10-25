pub struct DeviceTolerationOpt {
    pub effect: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub key: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub operator: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub toleration_seconds: <Option<i64> as crate::Optionable>::Optioned,
    pub value: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta2::DeviceToleration {
    type Optioned = DeviceTolerationOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceTolerationOpt {
    type Optioned = DeviceTolerationOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta2::DeviceToleration {
    fn into_optioned(self) -> DeviceTolerationOpt {
        DeviceTolerationOpt {
            effect: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.effect),
            key: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.key),
            operator: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.operator),
            toleration_seconds: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(self.toleration_seconds),
            value: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.value),
        }
    }
    fn try_from_optioned(
        value: DeviceTolerationOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            effect: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.effect)?,
            key: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.key)?,
            operator: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.operator)?,
            toleration_seconds: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(value.toleration_seconds)?,
            value: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.value)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceTolerationOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.effect, other.effect)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.key, other.key)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.operator, other.operator)?;
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(
            &mut self.toleration_seconds,
            other.toleration_seconds,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.value, other.value)?;
        Ok(())
    }
}
