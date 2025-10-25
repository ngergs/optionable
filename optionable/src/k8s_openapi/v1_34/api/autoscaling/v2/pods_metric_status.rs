pub struct PodsMetricStatusOpt {
    pub current: Option<
        <::k8s_openapi::api::autoscaling::v2::MetricValueStatus as crate::Optionable>::Optioned,
    >,
    pub metric: Option<
        <::k8s_openapi::api::autoscaling::v2::MetricIdentifier as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::autoscaling::v2::pods_metric_status::PodsMetricStatus {
    type Optioned = PodsMetricStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for PodsMetricStatusOpt {
    type Optioned = PodsMetricStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v2::pods_metric_status::PodsMetricStatus {
    fn into_optioned(self) -> PodsMetricStatusOpt {
        PodsMetricStatusOpt {
            current: Some(
                <::k8s_openapi::api::autoscaling::v2::MetricValueStatus as crate::OptionableConvert>::into_optioned(
                    self.current,
                ),
            ),
            metric: Some(
                <::k8s_openapi::api::autoscaling::v2::MetricIdentifier as crate::OptionableConvert>::into_optioned(
                    self.metric,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: PodsMetricStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            current: <::k8s_openapi::api::autoscaling::v2::MetricValueStatus as crate::OptionableConvert>::try_from_optioned(
                value
                    .current
                    .ok_or(crate::optionable::Error {
                        missing_field: "current",
                    })?,
            )?,
            metric: <::k8s_openapi::api::autoscaling::v2::MetricIdentifier as crate::OptionableConvert>::try_from_optioned(
                value
                    .metric
                    .ok_or(crate::optionable::Error {
                        missing_field: "metric",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PodsMetricStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.current {
            <::k8s_openapi::api::autoscaling::v2::MetricValueStatus as crate::OptionableConvert>::merge(
                &mut self.current,
                other_value,
            )?;
        }
        if let Some(other_value) = other.metric {
            <::k8s_openapi::api::autoscaling::v2::MetricIdentifier as crate::OptionableConvert>::merge(
                &mut self.metric,
                other_value,
            )?;
        }
        Ok(())
    }
}
