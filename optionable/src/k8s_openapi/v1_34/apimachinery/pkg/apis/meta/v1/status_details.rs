pub struct StatusDetailsOpt {
    pub causes: <Option<
        std::vec::Vec<::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusCause>,
    > as crate::Optionable>::Optioned,
    pub group: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub kind: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub retry_after_seconds: <Option<i32> as crate::Optionable>::Optioned,
    pub uid: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusDetails {
    type Optioned = StatusDetailsOpt;
}
#[automatically_derived]
impl crate::Optionable for StatusDetailsOpt {
    type Optioned = StatusDetailsOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusDetails {
    fn into_optioned(self) -> StatusDetailsOpt {
        StatusDetailsOpt {
            causes: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusCause,
                >,
            > as crate::OptionableConvert>::into_optioned(self.causes),
            group: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.group),
            kind: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.kind),
            name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.name),
            retry_after_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.retry_after_seconds),
            uid: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.uid),
        }
    }
    fn try_from_optioned(
        value: StatusDetailsOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            causes: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusCause,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.causes)?,
            group: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.group)?,
            kind: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.kind)?,
            name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.name)?,
            retry_after_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(
                value.retry_after_seconds,
            )?,
            uid: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.uid)?,
        })
    }
    fn merge(
        &mut self,
        other: StatusDetailsOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<::k8s_openapi::apimachinery::pkg::apis::meta::v1::StatusCause>,
        > as crate::OptionableConvert>::merge(&mut self.causes, other.causes)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.group, other.group)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.kind, other.kind)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.name, other.name)?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.retry_after_seconds,
            other.retry_after_seconds,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.uid, other.uid)?;
        Ok(())
    }
}
