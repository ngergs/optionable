#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ExternalMetricSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<
        <::k8s_openapi::api::autoscaling::v2::MetricIdentifier as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<
        <::k8s_openapi::api::autoscaling::v2::MetricTarget as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::autoscaling::v2::ExternalMetricSource {
    type Optioned = ExternalMetricSourceAc;
}
#[automatically_derived]
impl crate::Optionable for ExternalMetricSourceAc {
    type Optioned = ExternalMetricSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v2::ExternalMetricSource {
    fn into_optioned(self) -> ExternalMetricSourceAc {
        ExternalMetricSourceAc {
            metric: Some(crate::OptionableConvert::into_optioned(self.metric)),
            target: Some(crate::OptionableConvert::into_optioned(self.target)),
        }
    }
    fn try_from_optioned(value: ExternalMetricSourceAc) -> Result<Self, crate::Error> {
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
    fn merge(&mut self, other: ExternalMetricSourceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.metric {
            crate::OptionableConvert::merge(&mut self.metric, other_value)?;
        }
        if let Some(other_value) = other.target {
            crate::OptionableConvert::merge(&mut self.target, other_value)?;
        }
        Ok(())
    }
}
