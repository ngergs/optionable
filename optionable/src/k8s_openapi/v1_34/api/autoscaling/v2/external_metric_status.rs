#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ExternalMetricStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<
        <::k8s_openapi::api::autoscaling::v2::MetricValueStatus as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<
        <::k8s_openapi::api::autoscaling::v2::MetricIdentifier as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::autoscaling::v2::ExternalMetricStatus {
    type Optioned = ExternalMetricStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ExternalMetricStatusAc {
    type Optioned = ExternalMetricStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v2::ExternalMetricStatus {
    fn into_optioned(self) -> ExternalMetricStatusAc {
        ExternalMetricStatusAc {
            current: Some(crate::OptionableConvert::into_optioned(self.current)),
            metric: Some(crate::OptionableConvert::into_optioned(self.metric)),
        }
    }
    fn try_from_optioned(
        value: ExternalMetricStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            current: crate::OptionableConvert::try_from_optioned(
                value
                    .current
                    .ok_or(crate::optionable::Error {
                        missing_field: "current",
                    })?,
            )?,
            metric: crate::OptionableConvert::try_from_optioned(
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
        other: ExternalMetricStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.current {
            crate::OptionableConvert::merge(&mut self.current, other_value)?;
        }
        if let Some(other_value) = other.metric {
            crate::OptionableConvert::merge(&mut self.metric, other_value)?;
        }
        Ok(())
    }
}
