#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ContainerStateWaiting is a waiting state of a container.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContainerStateWaitingAc {
    /// Message regarding why the container is not yet running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// (brief) reason the container is not yet running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ContainerStateWaiting {
    type Optioned = ContainerStateWaitingAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerStateWaitingAc {
    type Optioned = ContainerStateWaitingAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ContainerStateWaiting {
    fn into_optioned(self) -> ContainerStateWaitingAc {
        ContainerStateWaitingAc {
            message: self.message,
            reason: self.reason,
        }
    }
    fn try_from_optioned(value: ContainerStateWaitingAc) -> Result<Self, crate::Error> {
        Ok(Self {
            message: value.message,
            reason: value.reason,
        })
    }
    fn merge(&mut self, other: ContainerStateWaitingAc) -> Result<(), crate::Error> {
        if self.message.is_none() {
            self.message = crate::OptionableConvert::try_from_optioned(other.message)?;
        } else {
            crate::OptionableConvert::merge(&mut self.message, other.message)?;
        }
        if self.reason.is_none() {
            self.reason = crate::OptionableConvert::try_from_optioned(other.reason)?;
        } else {
            crate::OptionableConvert::merge(&mut self.reason, other.reason)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ContainerStateWaiting>
for ContainerStateWaitingAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ContainerStateWaiting,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ContainerStateWaiting, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ContainerStateWaiting,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
