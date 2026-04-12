#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Spec to control the desired behavior of daemon set rolling update.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RollingUpdateDaemonSetAc {
    /// The maximum number of nodes with an existing available DaemonSet pod that can have an updated DaemonSet pod during during an update. Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%). This can not be 0 if MaxUnavailable is 0. Absolute number is calculated from percentage by rounding up to a minimum of 1. Default value is 0. Example: when this is set to 30%, at most 30% of the total number of nodes that should be running the daemon pod (i.e. status.desiredNumberScheduled) can have their a new pod created before the old pod is marked as deleted. The update starts by launching new pods on 30% of nodes. Once an updated pod is available (Ready for at least minReadySeconds) the old DaemonSet pod on that node is marked deleted. If the old pod becomes unavailable for any reason (Ready transitions to false, is evicted, or is drained) an updated pod is immediately created on that node without considering surge limits. Allowing surge implies the possibility that the resources consumed by the daemonset on any given node can double if the readiness check fails, and so resource intensive daemonsets should take into account that they may cause evictions during disruption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_surge: Option<
        <::k8s_openapi027::apimachinery::pkg::util::intstr::IntOrString as crate::Optionable>::Optioned,
    >,
    /// The maximum number of DaemonSet pods that can be unavailable during the update. Value can be an absolute number (ex: 5) or a percentage of total number of DaemonSet pods at the start of the update (ex: 10%). Absolute number is calculated from percentage by rounding up. This cannot be 0 if MaxSurge is 0 Default value is 1. Example: when this is set to 30%, at most 30% of the total number of nodes that should be running the daemon pod (i.e. status.desiredNumberScheduled) can have their pods stopped for an update at any given time. The update starts by stopping at most 30% of those DaemonSet pods and then brings up new DaemonSet pods in their place. Once the new pods are available, it then proceeds onto other DaemonSet pods, thus ensuring that at least 70% of original number of DaemonSet pods are available at all times during the update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<
        <::k8s_openapi027::apimachinery::pkg::util::intstr::IntOrString as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::apps::v1::RollingUpdateDaemonSet {
    type Optioned = RollingUpdateDaemonSetAc;
}
#[automatically_derived]
impl crate::Optionable for RollingUpdateDaemonSetAc {
    type Optioned = RollingUpdateDaemonSetAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::apps::v1::RollingUpdateDaemonSet {
    fn into_optioned(self) -> RollingUpdateDaemonSetAc {
        RollingUpdateDaemonSetAc {
            max_surge: crate::OptionableConvert::into_optioned(self.max_surge),
            max_unavailable: crate::OptionableConvert::into_optioned(
                self.max_unavailable,
            ),
        }
    }
    fn try_from_optioned(value: RollingUpdateDaemonSetAc) -> Result<Self, crate::Error> {
        Ok(Self {
            max_surge: crate::OptionableConvert::try_from_optioned(value.max_surge)?,
            max_unavailable: crate::OptionableConvert::try_from_optioned(
                value.max_unavailable,
            )?,
        })
    }
    fn merge(&mut self, other: RollingUpdateDaemonSetAc) -> Result<(), crate::Error> {
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
impl crate::OptionedConvert<k8s_openapi027::api::apps::v1::RollingUpdateDaemonSet>
for RollingUpdateDaemonSetAc {
    fn from_optionable(
        value: k8s_openapi027::api::apps::v1::RollingUpdateDaemonSet,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::apps::v1::RollingUpdateDaemonSet, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apps::v1::RollingUpdateDaemonSet,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
