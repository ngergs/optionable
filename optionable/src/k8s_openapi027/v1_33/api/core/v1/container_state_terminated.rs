#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ContainerStateTerminated is a terminated state of a container.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContainerStateTerminatedAc {
    /// Container's ID in the format '\<type\>://\<container_id\>'
    #[serde(rename = "containerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<std::string::String>,
    /// Exit status from the last termination of the container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    /// Time at which the container last terminated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// Message regarding the last termination of the container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// (brief) reason from the last termination of the container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
    /// Signal from the last termination of the container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal: Option<i32>,
    /// Time at which previous execution of the container started
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ContainerStateTerminated {
    type Optioned = ContainerStateTerminatedAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerStateTerminatedAc {
    type Optioned = ContainerStateTerminatedAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::ContainerStateTerminated {
    fn into_optioned(self) -> ContainerStateTerminatedAc {
        ContainerStateTerminatedAc {
            container_id: self.container_id,
            exit_code: Some(self.exit_code),
            finished_at: crate::OptionableConvert::into_optioned(self.finished_at),
            message: self.message,
            reason: self.reason,
            signal: self.signal,
            started_at: crate::OptionableConvert::into_optioned(self.started_at),
        }
    }
    fn try_from_optioned(
        value: ContainerStateTerminatedAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            container_id: value.container_id,
            exit_code: value
                .exit_code
                .ok_or(crate::Error {
                    missing_field: "exit_code",
                })?,
            finished_at: crate::OptionableConvert::try_from_optioned(value.finished_at)?,
            message: value.message,
            reason: value.reason,
            signal: value.signal,
            started_at: crate::OptionableConvert::try_from_optioned(value.started_at)?,
        })
    }
    fn merge(&mut self, other: ContainerStateTerminatedAc) -> Result<(), crate::Error> {
        if self.container_id.is_none() {
            self.container_id = other.container_id;
        }
        if let Some(other_value) = other.container_id {
            crate::OptionableConvert::merge(&mut self.container_id, other_value)?;
        }
        if let Some(other_value) = other.exit_code {
            self.exit_code = other_value;
        }
        if self.finished_at.is_none() {
            self.finished_at = other.finished_at;
        }
        if let Some(other_value) = other.finished_at {
            crate::OptionableConvert::merge(&mut self.finished_at, other_value)?;
        }
        if self.message.is_none() {
            self.message = other.message;
        }
        if let Some(other_value) = other.message {
            crate::OptionableConvert::merge(&mut self.message, other_value)?;
        }
        if self.reason.is_none() {
            self.reason = other.reason;
        }
        if let Some(other_value) = other.reason {
            crate::OptionableConvert::merge(&mut self.reason, other_value)?;
        }
        if self.signal.is_none() {
            self.signal = other.signal;
        }
        if let Some(other_value) = other.signal {
            crate::OptionableConvert::merge(&mut self.signal, other_value)?;
        }
        if self.started_at.is_none() {
            self.started_at = other.started_at;
        }
        if let Some(other_value) = other.started_at {
            crate::OptionableConvert::merge(&mut self.started_at, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ContainerStateTerminated>
for ContainerStateTerminatedAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ContainerStateTerminated,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ContainerStateTerminated, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ContainerStateTerminated,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
