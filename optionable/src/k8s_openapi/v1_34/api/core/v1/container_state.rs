pub struct ContainerStateOpt {
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
impl crate::Optionable
for ::k8s_openapi::api::core::v1::container_state::ContainerState {
    type Optioned = ContainerStateOpt;
}
#[automatically_derived]
impl crate::Optionable for ContainerStateOpt {
    type Optioned = ContainerStateOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::container_state::ContainerState {
    fn into_optioned(self) -> ContainerStateOpt {
        ContainerStateOpt {
            running: <Option<
                ::k8s_openapi::api::core::v1::ContainerStateRunning,
            > as crate::OptionableConvert>::into_optioned(self.running),
            terminated: <Option<
                ::k8s_openapi::api::core::v1::ContainerStateTerminated,
            > as crate::OptionableConvert>::into_optioned(self.terminated),
            waiting: <Option<
                ::k8s_openapi::api::core::v1::ContainerStateWaiting,
            > as crate::OptionableConvert>::into_optioned(self.waiting),
        }
    }
    fn try_from_optioned(
        value: ContainerStateOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            running: <Option<
                ::k8s_openapi::api::core::v1::ContainerStateRunning,
            > as crate::OptionableConvert>::try_from_optioned(value.running)?,
            terminated: <Option<
                ::k8s_openapi::api::core::v1::ContainerStateTerminated,
            > as crate::OptionableConvert>::try_from_optioned(value.terminated)?,
            waiting: <Option<
                ::k8s_openapi::api::core::v1::ContainerStateWaiting,
            > as crate::OptionableConvert>::try_from_optioned(value.waiting)?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerStateOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::core::v1::ContainerStateRunning,
        > as crate::OptionableConvert>::merge(&mut self.running, other.running)?;
        <Option<
            ::k8s_openapi::api::core::v1::ContainerStateTerminated,
        > as crate::OptionableConvert>::merge(&mut self.terminated, other.terminated)?;
        <Option<
            ::k8s_openapi::api::core::v1::ContainerStateWaiting,
        > as crate::OptionableConvert>::merge(&mut self.waiting, other.waiting)?;
        Ok(())
    }
}
