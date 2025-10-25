pub struct ObjectMetricStatusOpt {
    pub current: Option<
        <::k8s_openapi::api::autoscaling::v2::MetricValueStatus as crate::Optionable>::Optioned,
    >,
    pub described_object: Option<
        <::k8s_openapi::api::autoscaling::v2::CrossVersionObjectReference as crate::Optionable>::Optioned,
    >,
    pub metric: Option<
        <::k8s_openapi::api::autoscaling::v2::MetricIdentifier as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::autoscaling::v2::object_metric_status::ObjectMetricStatus {
    type Optioned = ObjectMetricStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for ObjectMetricStatusOpt {
    type Optioned = ObjectMetricStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v2::object_metric_status::ObjectMetricStatus {
    fn into_optioned(self) -> ObjectMetricStatusOpt {
        ObjectMetricStatusOpt {
            current: Some(
                <::k8s_openapi::api::autoscaling::v2::MetricValueStatus as crate::OptionableConvert>::into_optioned(
                    self.current,
                ),
            ),
            described_object: Some(
                <::k8s_openapi::api::autoscaling::v2::CrossVersionObjectReference as crate::OptionableConvert>::into_optioned(
                    self.described_object,
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
        value: ObjectMetricStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            current: <::k8s_openapi::api::autoscaling::v2::MetricValueStatus as crate::OptionableConvert>::try_from_optioned(
                value
                    .current
                    .ok_or(crate::optionable::Error {
                        missing_field: "current",
                    })?,
            )?,
            described_object: <::k8s_openapi::api::autoscaling::v2::CrossVersionObjectReference as crate::OptionableConvert>::try_from_optioned(
                value
                    .described_object
                    .ok_or(crate::optionable::Error {
                        missing_field: "described_object",
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
        other: ObjectMetricStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.current {
            <::k8s_openapi::api::autoscaling::v2::MetricValueStatus as crate::OptionableConvert>::merge(
                &mut self.current,
                other_value,
            )?;
        }
        if let Some(other_value) = other.described_object {
            <::k8s_openapi::api::autoscaling::v2::CrossVersionObjectReference as crate::OptionableConvert>::merge(
                &mut self.described_object,
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
