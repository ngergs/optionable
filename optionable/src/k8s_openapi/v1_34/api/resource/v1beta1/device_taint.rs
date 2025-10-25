pub struct DeviceTaintOpt {
    pub effect: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub key: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub time_added: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    pub value: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta1::DeviceTaint {
    type Optioned = DeviceTaintOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceTaintOpt {
    type Optioned = DeviceTaintOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1beta1::DeviceTaint {
    fn into_optioned(self) -> DeviceTaintOpt {
        DeviceTaintOpt {
            effect: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.effect,
                ),
            ),
            key: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.key,
                ),
            ),
            time_added: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::into_optioned(self.time_added),
            value: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.value),
        }
    }
    fn try_from_optioned(
        value: DeviceTaintOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            effect: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .effect
                    .ok_or(crate::optionable::Error {
                        missing_field: "effect",
                    })?,
            )?,
            key: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .key
                    .ok_or(crate::optionable::Error {
                        missing_field: "key",
                    })?,
            )?,
            time_added: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::try_from_optioned(value.time_added)?,
            value: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.value)?,
        })
    }
    fn merge(&mut self, other: DeviceTaintOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.effect {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.effect,
                other_value,
            )?;
        }
        if let Some(other_value) = other.key {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.key,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
        > as crate::OptionableConvert>::merge(&mut self.time_added, other.time_added)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.value, other.value)?;
        Ok(())
    }
}
