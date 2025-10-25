pub struct ContainerUserOpt {
    pub linux: <Option<
        ::k8s_openapi::api::core::v1::LinuxContainerUser,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::container_user::ContainerUser {
    type Optioned = ContainerUserOpt;
}
#[automatically_derived]
impl crate::Optionable for ContainerUserOpt {
    type Optioned = ContainerUserOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::container_user::ContainerUser {
    fn into_optioned(self) -> ContainerUserOpt {
        ContainerUserOpt {
            linux: <Option<
                ::k8s_openapi::api::core::v1::LinuxContainerUser,
            > as crate::OptionableConvert>::into_optioned(self.linux),
        }
    }
    fn try_from_optioned(
        value: ContainerUserOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            linux: <Option<
                ::k8s_openapi::api::core::v1::LinuxContainerUser,
            > as crate::OptionableConvert>::try_from_optioned(value.linux)?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerUserOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::core::v1::LinuxContainerUser,
        > as crate::OptionableConvert>::merge(&mut self.linux, other.linux)?;
        Ok(())
    }
}
