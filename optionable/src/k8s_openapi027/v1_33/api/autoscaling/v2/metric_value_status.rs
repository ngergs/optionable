#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// MetricValueStatus holds the current value for a metric
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MetricValueStatusAc {
    /// currentAverageUtilization is the current value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_utilization: Option<i32>,
    /// averageValue is the current value of the average of the metric across all relevant pods (as a quantity)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_value: Option<
        <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
    /// value is the current value of the metric (as a quantity).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<
        <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::autoscaling::v2::MetricValueStatus {
    type Optioned = MetricValueStatusAc;
}
#[automatically_derived]
impl crate::Optionable for MetricValueStatusAc {
    type Optioned = MetricValueStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::autoscaling::v2::MetricValueStatus {
    fn into_optioned(self) -> MetricValueStatusAc {
        MetricValueStatusAc {
            average_utilization: self.average_utilization,
            average_value: crate::OptionableConvert::into_optioned(self.average_value),
            value: crate::OptionableConvert::into_optioned(self.value),
        }
    }
    fn try_from_optioned(value: MetricValueStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            average_utilization: value.average_utilization,
            average_value: crate::OptionableConvert::try_from_optioned(
                value.average_value,
            )?,
            value: crate::OptionableConvert::try_from_optioned(value.value)?,
        })
    }
    fn merge(&mut self, other: MetricValueStatusAc) -> Result<(), crate::Error> {
        self.average_utilization = other.average_utilization;
        crate::OptionableConvert::merge(&mut self.average_value, other.average_value)?;
        crate::OptionableConvert::merge(&mut self.value, other.value)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::autoscaling::v2::MetricValueStatus>
for MetricValueStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::autoscaling::v2::MetricValueStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::autoscaling::v2::MetricValueStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::autoscaling::v2::MetricValueStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
