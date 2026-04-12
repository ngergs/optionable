#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ExecAction describes a "run in container" action.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExecActionAc {
    /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ExecAction {
    type Optioned = ExecActionAc;
}
#[automatically_derived]
impl crate::Optionable for ExecActionAc {
    type Optioned = ExecActionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ExecAction {
    fn into_optioned(self) -> ExecActionAc {
        ExecActionAc {
            command: self.command,
        }
    }
    fn try_from_optioned(value: ExecActionAc) -> Result<Self, crate::Error> {
        Ok(Self { command: value.command })
    }
    fn merge(&mut self, other: ExecActionAc) -> Result<(), crate::Error> {
        if self.command.is_none() {
            self.command = crate::OptionableConvert::try_from_optioned(other.command)?;
        } else if let Some(self_value) = self.command.as_mut()
            && let Some(other_value) = other.command
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ExecAction> for ExecActionAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::ExecAction) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ExecAction, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ExecAction,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
