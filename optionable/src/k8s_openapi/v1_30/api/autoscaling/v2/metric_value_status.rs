#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct MetricValueStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_utilization: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_value: <Option<
        ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: <Option<
        ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::autoscaling::v2::MetricValueStatus {
    type Optioned = MetricValueStatusAc;
}
#[automatically_derived]
impl crate::Optionable for MetricValueStatusAc {
    type Optioned = MetricValueStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v2::MetricValueStatus {
    fn into_optioned(self) -> MetricValueStatusAc {
        MetricValueStatusAc {
            average_utilization: crate::OptionableConvert::into_optioned(
                self.average_utilization,
            ),
            average_value: crate::OptionableConvert::into_optioned(self.average_value),
            value: crate::OptionableConvert::into_optioned(self.value),
        }
    }
    fn try_from_optioned(value: MetricValueStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            average_utilization: crate::OptionableConvert::try_from_optioned(
                value.average_utilization,
            )?,
            average_value: crate::OptionableConvert::try_from_optioned(
                value.average_value,
            )?,
            value: crate::OptionableConvert::try_from_optioned(value.value)?,
        })
    }
    fn merge(&mut self, other: MetricValueStatusAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.average_utilization,
            other.average_utilization,
        )?;
        crate::OptionableConvert::merge(&mut self.average_value, other.average_value)?;
        crate::OptionableConvert::merge(&mut self.value, other.value)?;
        Ok(())
    }
}
