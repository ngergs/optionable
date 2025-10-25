pub struct PodsMetricSourceOpt {
    pub metric: Option<
        <::k8s_openapi::api::autoscaling::v2::MetricIdentifier as crate::Optionable>::Optioned,
    >,
    pub target: Option<
        <::k8s_openapi::api::autoscaling::v2::MetricTarget as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::autoscaling::v2::pods_metric_source::PodsMetricSource {
    type Optioned = PodsMetricSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for PodsMetricSourceOpt {
    type Optioned = PodsMetricSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v2::pods_metric_source::PodsMetricSource {
    fn into_optioned(self) -> PodsMetricSourceOpt {
        PodsMetricSourceOpt {
            metric: Some(
                <::k8s_openapi::api::autoscaling::v2::MetricIdentifier as crate::OptionableConvert>::into_optioned(
                    self.metric,
                ),
            ),
            target: Some(
                <::k8s_openapi::api::autoscaling::v2::MetricTarget as crate::OptionableConvert>::into_optioned(
                    self.target,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: PodsMetricSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metric: <::k8s_openapi::api::autoscaling::v2::MetricIdentifier as crate::OptionableConvert>::try_from_optioned(
                value
                    .metric
                    .ok_or(crate::optionable::Error {
                        missing_field: "metric",
                    })?,
            )?,
            target: <::k8s_openapi::api::autoscaling::v2::MetricTarget as crate::OptionableConvert>::try_from_optioned(
                value
                    .target
                    .ok_or(crate::optionable::Error {
                        missing_field: "target",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PodsMetricSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.metric {
            <::k8s_openapi::api::autoscaling::v2::MetricIdentifier as crate::OptionableConvert>::merge(
                &mut self.metric,
                other_value,
            )?;
        }
        if let Some(other_value) = other.target {
            <::k8s_openapi::api::autoscaling::v2::MetricTarget as crate::OptionableConvert>::merge(
                &mut self.target,
                other_value,
            )?;
        }
        Ok(())
    }
}
