pub struct ContainerStateAc {
    pub running: <Option<
        ::k8s_openapi::api::core::v1::ContainerStateRunning,
    > as crate::Optionable>::Optioned,
    pub terminated: <Option<
        ::k8s_openapi::api::core::v1::ContainerStateTerminated,
    > as crate::Optionable>::Optioned,
    pub waiting: <Option<
        ::k8s_openapi::api::core::v1::ContainerStateWaiting,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ContainerState {
    type Optioned = ContainerStateAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerStateAc {
    type Optioned = ContainerStateAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ContainerState {
    fn into_optioned(self) -> ContainerStateAc {
        ContainerStateAc {
            running: crate::OptionableConvert::into_optioned(self.running),
            terminated: crate::OptionableConvert::into_optioned(self.terminated),
            waiting: crate::OptionableConvert::into_optioned(self.waiting),
        }
    }
    fn try_from_optioned(
        value: ContainerStateAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            running: crate::OptionableConvert::try_from_optioned(value.running)?,
            terminated: crate::OptionableConvert::try_from_optioned(value.terminated)?,
            waiting: crate::OptionableConvert::try_from_optioned(value.waiting)?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerStateAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.running, other.running)?;
        crate::OptionableConvert::merge(&mut self.terminated, other.terminated)?;
        crate::OptionableConvert::merge(&mut self.waiting, other.waiting)?;
        Ok(())
    }
}
