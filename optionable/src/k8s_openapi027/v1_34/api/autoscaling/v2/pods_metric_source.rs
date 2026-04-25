#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodsMetricSource indicates how to scale on a metric describing each pod in the current scale target (for example, transactions-processed-per-second). The values will be averaged together before being compared to the target value.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodsMetricSourceAc {
    /// metric identifies the target metric by name and selector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<
        <::k8s_openapi027::api::autoscaling::v2::MetricIdentifier as crate::Optionable>::Optioned,
    >,
    /// target specifies the target value for the given metric
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<
        <::k8s_openapi027::api::autoscaling::v2::MetricTarget as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::autoscaling::v2::PodsMetricSource {
    type Optioned = PodsMetricSourceAc;
}
#[automatically_derived]
impl crate::Optionable for PodsMetricSourceAc {
    type Optioned = PodsMetricSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::autoscaling::v2::PodsMetricSource {
    fn into_optioned(self) -> PodsMetricSourceAc {
        PodsMetricSourceAc {
            metric: Some(crate::OptionableConvert::into_optioned(self.metric)),
            target: Some(crate::OptionableConvert::into_optioned(self.target)),
        }
    }
    fn try_from_optioned(value: PodsMetricSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metric: crate::OptionableConvert::try_from_optioned(
                value
                    .metric
                    .ok_or(crate::Error {
                        missing_field: "metric",
                    })?,
            )?,
            target: crate::OptionableConvert::try_from_optioned(
                value
                    .target
                    .ok_or(crate::Error {
                        missing_field: "target",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: PodsMetricSourceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.metric {
            crate::OptionableConvert::merge(&mut self.metric, other_value)?;
        }
        if let Some(other_value) = other.target {
            crate::OptionableConvert::merge(&mut self.target, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::autoscaling::v2::PodsMetricSource>
for PodsMetricSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::autoscaling::v2::PodsMetricSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::autoscaling::v2::PodsMetricSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::autoscaling::v2::PodsMetricSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for PodsMetricSourceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.metric, other.metric);
        k8s_openapi027::DeepMerge::merge_from(&mut self.target, other.target);
    }
}
