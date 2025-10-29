pub struct StatusCauseOpt {
    pub field: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusCause {
    type Optioned = StatusCauseOpt;
}
#[automatically_derived]
impl crate::Optionable for StatusCauseOpt {
    type Optioned = StatusCauseOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusCause {
    fn into_optioned(self) -> StatusCauseOpt {
        StatusCauseOpt {
            field: crate::OptionableConvert::into_optioned(self.field),
            message: crate::OptionableConvert::into_optioned(self.message),
            reason: crate::OptionableConvert::into_optioned(self.reason),
        }
    }
    fn try_from_optioned(
        value: StatusCauseOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            field: crate::OptionableConvert::try_from_optioned(value.field)?,
            message: crate::OptionableConvert::try_from_optioned(value.message)?,
            reason: crate::OptionableConvert::try_from_optioned(value.reason)?,
        })
    }
    fn merge(&mut self, other: StatusCauseOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.field, other.field)?;
        crate::OptionableConvert::merge(&mut self.message, other.message)?;
        crate::OptionableConvert::merge(&mut self.reason, other.reason)?;
        Ok(())
    }
}
