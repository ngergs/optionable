#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ContainerUserAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux: <Option<
        ::k8s_openapi::api::core::v1::LinuxContainerUser,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ContainerUser {
    type Optioned = ContainerUserAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerUserAc {
    type Optioned = ContainerUserAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ContainerUser {
    fn into_optioned(self) -> ContainerUserAc {
        ContainerUserAc {
            linux: crate::OptionableConvert::into_optioned(self.linux),
        }
    }
    fn try_from_optioned(
        value: ContainerUserAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            linux: crate::OptionableConvert::try_from_optioned(value.linux)?,
        })
    }
    fn merge(&mut self, other: ContainerUserAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.linux, other.linux)?;
        Ok(())
    }
}
