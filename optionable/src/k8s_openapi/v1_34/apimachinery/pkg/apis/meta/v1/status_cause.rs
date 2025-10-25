pub struct StatusCauseOpt {
    pub field: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::status_cause::StatusCause {
    type Optioned = StatusCauseOpt;
}
#[automatically_derived]
impl crate::Optionable for StatusCauseOpt {
    type Optioned = StatusCauseOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::status_cause::StatusCause {
    fn into_optioned(self) -> StatusCauseOpt {
        StatusCauseOpt {
            field: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.field),
            message: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.message),
            reason: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.reason),
        }
    }
    fn try_from_optioned(
        value: StatusCauseOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            field: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.field)?,
            message: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.message)?,
            reason: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.reason)?,
        })
    }
    fn merge(&mut self, other: StatusCauseOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.field, other.field)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.message, other.message)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.reason, other.reason)?;
        Ok(())
    }
}
