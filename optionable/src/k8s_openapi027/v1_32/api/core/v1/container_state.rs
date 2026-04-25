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
        } else if let Some(self_value) = self.running.as_mut()
            && let Some(other_value) = other.running
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.terminated.is_none() {
            self.terminated = crate::OptionableConvert::try_from_optioned(
                other.terminated,
            )?;
        } else if let Some(self_value) = self.terminated.as_mut()
            && let Some(other_value) = other.terminated
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.waiting.is_none() {
            self.waiting = crate::OptionableConvert::try_from_optioned(other.waiting)?;
        } else if let Some(self_value) = self.waiting.as_mut()
            && let Some(other_value) = other.waiting
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
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
impl k8s_openapi027::DeepMerge for ContainerStateAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.running, other.running);
        k8s_openapi027::DeepMerge::merge_from(&mut self.terminated, other.terminated);
        k8s_openapi027::DeepMerge::merge_from(&mut self.waiting, other.waiting);
    }
}
