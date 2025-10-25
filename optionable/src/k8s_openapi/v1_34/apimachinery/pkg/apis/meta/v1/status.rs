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
            code: <Option<i32> as crate::OptionableConvert>::into_optioned(self.code),
            details: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusDetails,
            > as crate::OptionableConvert>::into_optioned(self.details),
            message: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.message),
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ListMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            reason: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.reason),
            status: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.status),
        }
    }
    fn try_from_optioned(value: StatusOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            code: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.code)?,
            details: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusDetails,
            > as crate::OptionableConvert>::try_from_optioned(value.details)?,
            message: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.message)?,
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ListMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            reason: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.reason)?,
            status: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.status)?,
        })
    }
    fn merge(&mut self, other: StatusOpt) -> Result<(), crate::optionable::Error> {
        <Option<i32> as crate::OptionableConvert>::merge(&mut self.code, other.code)?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusDetails,
        > as crate::OptionableConvert>::merge(&mut self.details, other.details)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.message, other.message)?;
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ListMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.reason, other.reason)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
