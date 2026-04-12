#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Spec to control the desired behavior of rolling update.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RollingUpdateDeploymentAc {
    /// The maximum number of pods that can be scheduled above the desired number of pods. Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%). This can not be 0 if MaxUnavailable is 0. Absolute number is calculated from percentage by rounding up. Defaults to 25%. Example: when this is set to 30%, the new ReplicaSet can be scaled up immediately when the rolling update starts, such that the total number of old and new pods do not exceed 130% of desired pods. Once old pods have been killed, new ReplicaSet can be scaled up further, ensuring that total number of pods running at any time during the update is at most 130% of desired pods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_surge: Option<
        <::k8s_openapi027::apimachinery::pkg::util::intstr::IntOrString as crate::Optionable>::Optioned,
    >,
    /// The maximum number of pods that can be unavailable during the update. Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%). Absolute number is calculated from percentage by rounding down. This can not be 0 if MaxSurge is 0. Defaults to 25%. Example: when this is set to 30%, the old ReplicaSet can be scaled down to 70% of desired pods immediately when the rolling update starts. Once new pods are ready, old ReplicaSet can be scaled down further, followed by scaling up the new ReplicaSet, ensuring that the total number of pods available at all times during the update is at least 70% of desired pods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<
        <::k8s_openapi027::apimachinery::pkg::util::intstr::IntOrString as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::apps::v1::RollingUpdateDeployment {
    type Optioned = RollingUpdateDeploymentAc;
}
#[automatically_derived]
impl crate::Optionable for RollingUpdateDeploymentAc {
    type Optioned = RollingUpdateDeploymentAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::apps::v1::RollingUpdateDeployment {
    fn into_optioned(self) -> RollingUpdateDeploymentAc {
        RollingUpdateDeploymentAc {
            max_surge: crate::OptionableConvert::into_optioned(self.max_surge),
            max_unavailable: crate::OptionableConvert::into_optioned(
                self.max_unavailable,
            ),
        }
    }
    fn try_from_optioned(
        value: RollingUpdateDeploymentAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            max_surge: crate::OptionableConvert::try_from_optioned(value.max_surge)?,
            max_unavailable: crate::OptionableConvert::try_from_optioned(
                value.max_unavailable,
            )?,
        })
    }
    fn merge(&mut self, other: RollingUpdateDeploymentAc) -> Result<(), crate::Error> {
        if self.max_surge.is_none() {
            self.max_surge = other.max_surge;
        }
        if let Some(other_value) = other.max_surge {
            crate::OptionableConvert::merge(&mut self.max_surge, other_value)?;
        }
        if self.max_unavailable.is_none() {
            self.max_unavailable = other.max_unavailable;
        }
        if let Some(other_value) = other.max_unavailable {
            crate::OptionableConvert::merge(&mut self.max_unavailable, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::apps::v1::RollingUpdateDeployment>
for RollingUpdateDeploymentAc {
    fn from_optionable(
        value: k8s_openapi027::api::apps::v1::RollingUpdateDeployment,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::apps::v1::RollingUpdateDeployment, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apps::v1::RollingUpdateDeployment,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
