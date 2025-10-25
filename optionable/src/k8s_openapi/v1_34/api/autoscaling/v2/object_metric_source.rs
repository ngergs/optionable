pub struct ObjectMetricSourceOpt {
    pub described_object: Option<
        <::k8s_openapi::api::autoscaling::v2::CrossVersionObjectReference as crate::Optionable>::Optioned,
    >,
    pub metric: Option<
        <::k8s_openapi::api::autoscaling::v2::MetricIdentifier as crate::Optionable>::Optioned,
    >,
    pub target: Option<
        <::k8s_openapi::api::autoscaling::v2::MetricTarget as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::autoscaling::v2::object_metric_source::ObjectMetricSource {
    type Optioned = ObjectMetricSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for ObjectMetricSourceOpt {
    type Optioned = ObjectMetricSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v2::object_metric_source::ObjectMetricSource {
    fn into_optioned(self) -> ObjectMetricSourceOpt {
        ObjectMetricSourceOpt {
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
            target: Some(
                <::k8s_openapi::api::autoscaling::v2::MetricTarget as crate::OptionableConvert>::into_optioned(
                    self.target,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: ObjectMetricSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
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
        other: ObjectMetricSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
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
        if let Some(other_value) = other.target {
            <::k8s_openapi::api::autoscaling::v2::MetricTarget as crate::OptionableConvert>::merge(
                &mut self.target,
                other_value,
            )?;
        }
        Ok(())
    }
}
