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
            self.container_id = crate::OptionableConvert::try_from_optioned(
                other.container_id,
            )?;
        } else if let Some(self_value) = self.container_id.as_mut()
            && let Some(other_value) = other.container_id
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.exit_code {
            self.exit_code = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.finished_at.is_none() {
            self.finished_at = crate::OptionableConvert::try_from_optioned(
                other.finished_at,
            )?;
        } else if let Some(self_value) = self.finished_at.as_mut()
            && let Some(other_value) = other.finished_at
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.message.is_none() {
            self.message = crate::OptionableConvert::try_from_optioned(other.message)?;
        } else if let Some(self_value) = self.message.as_mut()
            && let Some(other_value) = other.message
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.reason.is_none() {
            self.reason = crate::OptionableConvert::try_from_optioned(other.reason)?;
        } else if let Some(self_value) = self.reason.as_mut()
            && let Some(other_value) = other.reason
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.signal.is_none() {
            self.signal = crate::OptionableConvert::try_from_optioned(other.signal)?;
        } else if let Some(self_value) = self.signal.as_mut()
            && let Some(other_value) = other.signal
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.started_at.is_none() {
            self.started_at = crate::OptionableConvert::try_from_optioned(
                other.started_at,
            )?;
        } else if let Some(self_value) = self.started_at.as_mut()
            && let Some(other_value) = other.started_at
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
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
