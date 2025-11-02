#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodsMetricSourceAc {
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
impl crate::Optionable for ::k8s_openapi::api::autoscaling::v2::PodsMetricSource {
    type Optioned = PodsMetricSourceAc;
}
#[automatically_derived]
impl crate::Optionable for PodsMetricSourceAc {
    type Optioned = PodsMetricSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::autoscaling::v2::PodsMetricSource {
    fn into_optioned(self) -> PodsMetricSourceAc {
        PodsMetricSourceAc {
            metric: Some(crate::OptionableConvert::into_optioned(self.metric)),
            target: Some(crate::OptionableConvert::into_optioned(self.target)),
        }
    }
    fn try_from_optioned(
        value: PodsMetricSourceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metric: crate::OptionableConvert::try_from_optioned(
                value
                    .metric
                    .ok_or(crate::optionable::Error {
                        missing_field: "metric",
                    })?,
            )?,
            target: crate::OptionableConvert::try_from_optioned(
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
        other: PodsMetricSourceAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.metric {
            crate::OptionableConvert::merge(&mut self.metric, other_value)?;
        }
        if let Some(other_value) = other.target {
            crate::OptionableConvert::merge(&mut self.target, other_value)?;
        }
        Ok(())
    }
}
