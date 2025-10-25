pub struct ComponentConditionOpt {
    pub error: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub status: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::component_condition::ComponentCondition {
    type Optioned = ComponentConditionOpt;
}
#[automatically_derived]
impl crate::Optionable for ComponentConditionOpt {
    type Optioned = ComponentConditionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::component_condition::ComponentCondition {
    fn into_optioned(self) -> ComponentConditionOpt {
        ComponentConditionOpt {
            error: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.error),
            message: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.message),
            status: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.status,
                ),
            ),
            type_: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.type_,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: ComponentConditionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            error: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.error)?,
            message: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.message)?,
            status: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .status
                    .ok_or(crate::optionable::Error {
                        missing_field: "status",
                    })?,
            )?,
            type_: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::optionable::Error {
                        missing_field: "type_",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ComponentConditionOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.error, other.error)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.message, other.message)?;
        if let Some(other_value) = other.status {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.status,
                other_value,
            )?;
        }
        if let Some(other_value) = other.type_ {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.type_,
                other_value,
            )?;
        }
        Ok(())
    }
}
