pub struct ContainerStateRunningOpt {
    pub started_at: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::container_state_running::ContainerStateRunning {
    type Optioned = ContainerStateRunningOpt;
}
#[automatically_derived]
impl crate::Optionable for ContainerStateRunningOpt {
    type Optioned = ContainerStateRunningOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::container_state_running::ContainerStateRunning {
    fn into_optioned(self) -> ContainerStateRunningOpt {
        ContainerStateRunningOpt {
            started_at: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::into_optioned(self.started_at),
        }
    }
    fn try_from_optioned(
        value: ContainerStateRunningOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            started_at: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::try_from_optioned(value.started_at)?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerStateRunningOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
        > as crate::OptionableConvert>::merge(&mut self.started_at, other.started_at)?;
        Ok(())
    }
}
