#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMetricSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub described_object: Option<
        <::k8s_openapi::api::autoscaling::v2::CrossVersionObjectReference as crate::Optionable>::Optioned,
    >,
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
impl crate::Optionable for ::k8s_openapi::api::autoscaling::v2::ObjectMetricSource {
    type Optioned = ObjectMetricSourceAc;
}
#[automatically_derived]
impl crate::Optionable for ObjectMetricSourceAc {
    type Optioned = ObjectMetricSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v2::ObjectMetricSource {
    fn into_optioned(self) -> ObjectMetricSourceAc {
        ObjectMetricSourceAc {
            described_object: Some(
                crate::OptionableConvert::into_optioned(self.described_object),
            ),
            metric: Some(crate::OptionableConvert::into_optioned(self.metric)),
            target: Some(crate::OptionableConvert::into_optioned(self.target)),
        }
    }
    fn try_from_optioned(value: ObjectMetricSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            described_object: crate::OptionableConvert::try_from_optioned(
                value
                    .described_object
                    .ok_or(crate::Error {
                        missing_field: "described_object",
                    })?,
            )?,
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
    fn merge(&mut self, other: ObjectMetricSourceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.described_object {
            crate::OptionableConvert::merge(&mut self.described_object, other_value)?;
        }
        if let Some(other_value) = other.metric {
            crate::OptionableConvert::merge(&mut self.metric, other_value)?;
        }
        if let Some(other_value) = other.target {
            crate::OptionableConvert::merge(&mut self.target, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::autoscaling::v2::ObjectMetricSource>
for ObjectMetricSourceAc {
    fn from_optionable(
        value: ::k8s_openapi::api::autoscaling::v2::ObjectMetricSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::autoscaling::v2::ObjectMetricSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::autoscaling::v2::ObjectMetricSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
