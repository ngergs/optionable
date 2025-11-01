pub struct ContainerStateTerminatedAc {
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
    type Optioned = ContainerStateTerminatedAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerStateTerminatedAc {
    type Optioned = ContainerStateTerminatedAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::ContainerStateTerminated {
    fn into_optioned(self) -> ContainerStateTerminatedAc {
        ContainerStateTerminatedAc {
            container_id: crate::OptionableConvert::into_optioned(self.container_id),
            exit_code: Some(self.exit_code),
            finished_at: crate::OptionableConvert::into_optioned(self.finished_at),
            message: crate::OptionableConvert::into_optioned(self.message),
            reason: crate::OptionableConvert::into_optioned(self.reason),
            signal: crate::OptionableConvert::into_optioned(self.signal),
            started_at: crate::OptionableConvert::into_optioned(self.started_at),
        }
    }
    fn try_from_optioned(
        value: ContainerStateTerminatedAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            container_id: crate::OptionableConvert::try_from_optioned(
                value.container_id,
            )?,
            exit_code: value
                .exit_code
                .ok_or(crate::optionable::Error {
                    missing_field: "exit_code",
                })?,
            finished_at: crate::OptionableConvert::try_from_optioned(value.finished_at)?,
            message: crate::OptionableConvert::try_from_optioned(value.message)?,
            reason: crate::OptionableConvert::try_from_optioned(value.reason)?,
            signal: crate::OptionableConvert::try_from_optioned(value.signal)?,
            started_at: crate::OptionableConvert::try_from_optioned(value.started_at)?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerStateTerminatedAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.container_id, other.container_id)?;
        if let Some(other_value) = other.exit_code {
            self.exit_code = other_value;
        }
        crate::OptionableConvert::merge(&mut self.finished_at, other.finished_at)?;
        crate::OptionableConvert::merge(&mut self.message, other.message)?;
        crate::OptionableConvert::merge(&mut self.reason, other.reason)?;
        crate::OptionableConvert::merge(&mut self.signal, other.signal)?;
        crate::OptionableConvert::merge(&mut self.started_at, other.started_at)?;
        Ok(())
    }
}
