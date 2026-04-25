#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ContainerStateRunning is a running state of a container.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContainerStateRunningAc {
    /// Time at which the container was last (re-)started
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ContainerStateRunning {
    type Optioned = ContainerStateRunningAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerStateRunningAc {
    type Optioned = ContainerStateRunningAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ContainerStateRunning {
    fn into_optioned(self) -> ContainerStateRunningAc {
        ContainerStateRunningAc {
            started_at: crate::OptionableConvert::into_optioned(self.started_at),
        }
    }
    fn try_from_optioned(value: ContainerStateRunningAc) -> Result<Self, crate::Error> {
        Ok(Self {
            started_at: crate::OptionableConvert::try_from_optioned(value.started_at)?,
        })
    }
    fn merge(&mut self, other: ContainerStateRunningAc) -> Result<(), crate::Error> {
        if self.started_at.is_none() {
            self.started_at = crate::OptionableConvert::try_from_optioned(
                other.started_at,
            )?;
        } else if let Some(self_value) = self.started_at.as_mut()
            && let Some(other_value) = other.started_at
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ContainerStateRunning>
for ContainerStateRunningAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ContainerStateRunning,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ContainerStateRunning, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ContainerStateRunning,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for ContainerStateRunningAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.started_at, other.started_at);
    }
}
