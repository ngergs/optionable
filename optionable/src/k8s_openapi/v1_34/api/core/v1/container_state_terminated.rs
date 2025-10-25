pub struct ContainerStateTerminatedOpt {
    pub container_id: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub exit_code: Option<i32>,
    pub finished_at: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub signal: <Option<i32> as crate::Optionable>::Optioned,
    pub started_at: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ContainerStateTerminated {
    type Optioned = ContainerStateTerminatedOpt;
}
#[automatically_derived]
impl crate::Optionable for ContainerStateTerminatedOpt {
    type Optioned = ContainerStateTerminatedOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::ContainerStateTerminated {
    fn into_optioned(self) -> ContainerStateTerminatedOpt {
        ContainerStateTerminatedOpt {
            container_id: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.container_id),
            exit_code: Some(self.exit_code),
            finished_at: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::into_optioned(self.finished_at),
            message: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.message),
            reason: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.reason),
            signal: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.signal),
            started_at: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::into_optioned(self.started_at),
        }
    }
    fn try_from_optioned(
        value: ContainerStateTerminatedOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            container_id: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.container_id)?,
            exit_code: value
                .exit_code
                .ok_or(crate::optionable::Error {
                    missing_field: "exit_code",
                })?,
            finished_at: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::try_from_optioned(value.finished_at)?,
            message: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.message)?,
            reason: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.reason)?,
            signal: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.signal)?,
            started_at: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::try_from_optioned(value.started_at)?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerStateTerminatedOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.container_id,
            other.container_id,
        )?;
        if let Some(other_value) = other.exit_code {
            self.exit_code = other_value;
        }
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
        > as crate::OptionableConvert>::merge(&mut self.finished_at, other.finished_at)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.message, other.message)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.reason, other.reason)?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(&mut self.signal, other.signal)?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
        > as crate::OptionableConvert>::merge(&mut self.started_at, other.started_at)?;
        Ok(())
    }
}
