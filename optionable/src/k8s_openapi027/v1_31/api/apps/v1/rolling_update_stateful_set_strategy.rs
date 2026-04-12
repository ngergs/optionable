#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// RollingUpdateStatefulSetStrategy is used to communicate parameter for RollingUpdateStatefulSetStrategyType.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RollingUpdateStatefulSetStrategyAc {
    /// The maximum number of pods that can be unavailable during the update. Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%). Absolute number is calculated from percentage by rounding up. This can not be 0. Defaults to 1. This field is alpha-level and is only honored by servers that enable the MaxUnavailableStatefulSet feature. The field applies to all pods in the range 0 to Replicas-1. That means if there is any unavailable pod in the range 0 to Replicas-1, it will be counted towards MaxUnavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<
        <::k8s_openapi027::apimachinery::pkg::util::intstr::IntOrString as crate::Optionable>::Optioned,
    >,
    /// Partition indicates the ordinal at which the StatefulSet should be partitioned for updates. During a rolling update, all pods from ordinal Replicas-1 to Partition are updated. All pods from ordinal Partition-1 to 0 remain untouched. This is helpful in being able to do a canary based deployment. The default value is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::apps::v1::RollingUpdateStatefulSetStrategy {
    type Optioned = RollingUpdateStatefulSetStrategyAc;
}
#[automatically_derived]
impl crate::Optionable for RollingUpdateStatefulSetStrategyAc {
    type Optioned = RollingUpdateStatefulSetStrategyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::apps::v1::RollingUpdateStatefulSetStrategy {
    fn into_optioned(self) -> RollingUpdateStatefulSetStrategyAc {
        RollingUpdateStatefulSetStrategyAc {
            max_unavailable: crate::OptionableConvert::into_optioned(
                self.max_unavailable,
            ),
            partition: self.partition,
        }
    }
    fn try_from_optioned(
        value: RollingUpdateStatefulSetStrategyAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            max_unavailable: crate::OptionableConvert::try_from_optioned(
                value.max_unavailable,
            )?,
            partition: value.partition,
        })
    }
    fn merge(
        &mut self,
        other: RollingUpdateStatefulSetStrategyAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.max_unavailable,
            other.max_unavailable,
        )?;
        if other.partition.is_some() {
            self.partition = other.partition;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::apps::v1::RollingUpdateStatefulSetStrategy,
> for RollingUpdateStatefulSetStrategyAc {
    fn from_optionable(
        value: k8s_openapi027::api::apps::v1::RollingUpdateStatefulSetStrategy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::apps::v1::RollingUpdateStatefulSetStrategy,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apps::v1::RollingUpdateStatefulSetStrategy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
