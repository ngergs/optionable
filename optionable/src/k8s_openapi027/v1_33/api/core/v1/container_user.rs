#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ContainerUser represents user identity information
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContainerUserAc {
    /// Linux holds user identity information initially attached to the first process of the containers in Linux. Note that the actual running identity can be changed if the process has enough privilege to do so.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux: Option<
        <::k8s_openapi027::api::core::v1::LinuxContainerUser as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ContainerUser {
    type Optioned = ContainerUserAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerUserAc {
    type Optioned = ContainerUserAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ContainerUser {
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
        if self.linux.is_none() {
            self.linux = crate::OptionableConvert::try_from_optioned(other.linux)?;
        } else if let Some(self_value) = self.linux.as_mut()
            && let Some(other_value) = other.linux
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ContainerUser>
for ContainerUserAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::ContainerUser) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ContainerUser, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ContainerUser,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
