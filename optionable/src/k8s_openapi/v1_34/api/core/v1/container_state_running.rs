pub struct ContainerStateRunningAc {
    pub started_at: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ContainerStateRunning {
    type Optioned = ContainerStateRunningAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerStateRunningAc {
    type Optioned = ContainerStateRunningAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ContainerStateRunning {
    fn into_optioned(self) -> ContainerStateRunningAc {
        ContainerStateRunningAc {
            started_at: crate::OptionableConvert::into_optioned(self.started_at),
        }
    }
    fn try_from_optioned(
        value: ContainerStateRunningAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            started_at: crate::OptionableConvert::try_from_optioned(value.started_at)?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerStateRunningAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.started_at, other.started_at)?;
        Ok(())
    }
}
