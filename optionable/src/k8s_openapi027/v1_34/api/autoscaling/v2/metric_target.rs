#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// MetricTarget defines the target value, average value, or average utilization of a specific metric
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MetricTargetAc {
    /// averageUtilization is the target value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods. Currently only valid for Resource metric source type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_utilization: Option<i32>,
    /// averageValue is the target value of the average of the metric across all relevant pods (as a quantity)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_value: Option<
        <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
    /// type represents whether the metric type is Utilization, Value, or AverageValue
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
    /// value is the target value of the metric (as a quantity).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<
        <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::autoscaling::v2::MetricTarget {
    type Optioned = MetricTargetAc;
}
#[automatically_derived]
impl crate::Optionable for MetricTargetAc {
    type Optioned = MetricTargetAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::autoscaling::v2::MetricTarget {
    fn into_optioned(self) -> MetricTargetAc {
        MetricTargetAc {
            average_utilization: self.average_utilization,
            average_value: crate::OptionableConvert::into_optioned(self.average_value),
            type_: Some(self.type_),
            value: crate::OptionableConvert::into_optioned(self.value),
        }
    }
    fn try_from_optioned(value: MetricTargetAc) -> Result<Self, crate::Error> {
        Ok(Self {
            average_utilization: value.average_utilization,
            average_value: crate::OptionableConvert::try_from_optioned(
                value.average_value,
            )?,
            type_: value
                .type_
                .ok_or(crate::Error {
                    missing_field: "type_",
                })?,
            value: crate::OptionableConvert::try_from_optioned(value.value)?,
        })
    }
    fn merge(&mut self, other: MetricTargetAc) -> Result<(), crate::Error> {
        if self.average_utilization.is_none() {
            self.average_utilization = crate::OptionableConvert::try_from_optioned(
                other.average_utilization,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.average_utilization,
                other.average_utilization,
            )?;
        }
        if self.average_value.is_none() {
            self.average_value = crate::OptionableConvert::try_from_optioned(
                other.average_value,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.average_value,
                other.average_value,
            )?;
        }
        if let Some(other_value) = other.type_ {
            self.type_ = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.value.is_none() {
            self.value = crate::OptionableConvert::try_from_optioned(other.value)?;
        } else {
            crate::OptionableConvert::merge(&mut self.value, other.value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::autoscaling::v2::MetricTarget>
for MetricTargetAc {
    fn from_optionable(
        value: k8s_openapi027::api::autoscaling::v2::MetricTarget,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::autoscaling::v2::MetricTarget, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::autoscaling::v2::MetricTarget,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
