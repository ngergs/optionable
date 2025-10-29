pub struct StatusOpt {
    pub code: <Option<i32> as crate::Optionable>::Optioned,
    pub details: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusDetails,
    > as crate::Optionable>::Optioned,
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ListMeta as crate::Optionable>::Optioned,
    >,
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub status: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Status {
    type Optioned = StatusOpt;
}
#[automatically_derived]
impl crate::Optionable for StatusOpt {
    type Optioned = StatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Status {
    fn into_optioned(self) -> StatusOpt {
        StatusOpt {
            code: crate::OptionableConvert::into_optioned(self.code),
            details: crate::OptionableConvert::into_optioned(self.details),
            message: crate::OptionableConvert::into_optioned(self.message),
            metadata: Some(crate::OptionableConvert::into_optioned(self.metadata)),
            reason: crate::OptionableConvert::into_optioned(self.reason),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(value: StatusOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            code: crate::OptionableConvert::try_from_optioned(value.code)?,
            details: crate::OptionableConvert::try_from_optioned(value.details)?,
            message: crate::OptionableConvert::try_from_optioned(value.message)?,
            metadata: crate::OptionableConvert::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            reason: crate::OptionableConvert::try_from_optioned(value.reason)?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(&mut self, other: StatusOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.code, other.code)?;
        crate::OptionableConvert::merge(&mut self.details, other.details)?;
        crate::OptionableConvert::merge(&mut self.message, other.message)?;
        if let Some(other_value) = other.metadata {
            crate::OptionableConvert::merge(&mut self.metadata, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.reason, other.reason)?;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
