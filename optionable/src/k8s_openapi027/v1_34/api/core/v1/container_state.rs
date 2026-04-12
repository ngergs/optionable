#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ContainerState holds a possible state of container. Only one of its members may be specified. If none of them is specified, the default one is ContainerStateWaiting.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContainerStateAc {
    /// Details about a running container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<
        <::k8s_openapi027::api::core::v1::ContainerStateRunning as crate::Optionable>::Optioned,
    >,
    /// Details about a terminated container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminated: Option<
        <::k8s_openapi027::api::core::v1::ContainerStateTerminated as crate::Optionable>::Optioned,
    >,
    /// Details about a waiting container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiting: Option<
        <::k8s_openapi027::api::core::v1::ContainerStateWaiting as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ContainerState {
    type Optioned = ContainerStateAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerStateAc {
    type Optioned = ContainerStateAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ContainerState {
    fn into_optioned(self) -> ContainerStateAc {
        ContainerStateAc {
            running: crate::OptionableConvert::into_optioned(self.running),
            terminated: crate::OptionableConvert::into_optioned(self.terminated),
            waiting: crate::OptionableConvert::into_optioned(self.waiting),
        }
    }
    fn try_from_optioned(value: ContainerStateAc) -> Result<Self, crate::Error> {
        Ok(Self {
            running: crate::OptionableConvert::try_from_optioned(value.running)?,
            terminated: crate::OptionableConvert::try_from_optioned(value.terminated)?,
            waiting: crate::OptionableConvert::try_from_optioned(value.waiting)?,
        })
    }
    fn merge(&mut self, other: ContainerStateAc) -> Result<(), crate::Error> {
        if self.running.is_none() {
            self.running = crate::OptionableConvert::try_from_optioned(other.running)?;
        } else {
            crate::OptionableConvert::merge(&mut self.running, other.running)?;
        }
        if self.terminated.is_none() {
            self.terminated = crate::OptionableConvert::try_from_optioned(
                other.terminated,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.terminated, other.terminated)?;
        }
        if self.waiting.is_none() {
            self.waiting = crate::OptionableConvert::try_from_optioned(other.waiting)?;
        } else {
            crate::OptionableConvert::merge(&mut self.waiting, other.waiting)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ContainerState>
for ContainerStateAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::ContainerState) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ContainerState, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ContainerState,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
