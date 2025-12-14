#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ContainerUser {
    fn into_optioned(self) -> ContainerUserAc {
        ContainerUserAc {
            linux: crate::OptionableConvert::into_optioned(self.linux),
        }
    }
    fn try_from_optioned(value: ContainerUserAc) -> Result<Self, crate::Error> {
        Ok(Self {
            linux: crate::OptionableConvert::try_from_optioned(value.linux)?,
        })
    }
    fn merge(&mut self, other: ContainerUserAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.linux, other.linux)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::ContainerUser>
for ContainerUserAc {
    fn from_optionable(value: ::k8s_openapi::api::core::v1::ContainerUser) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::ContainerUser, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::ContainerUser,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
