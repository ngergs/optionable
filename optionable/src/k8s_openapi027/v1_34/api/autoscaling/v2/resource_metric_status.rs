#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ResourceMetricStatus indicates the current value of a resource metric known to Kubernetes, as specified in requests and limits, describing each pod in the current scale target (e.g. CPU or memory).  Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceMetricStatusAc {
    /// current contains the current value for the given metric
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<
        <::k8s_openapi027::api::autoscaling::v2::MetricValueStatus as crate::Optionable>::Optioned,
    >,
    /// name is the name of the resource in question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::autoscaling::v2::ResourceMetricStatus {
    type Optioned = ResourceMetricStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceMetricStatusAc {
    type Optioned = ResourceMetricStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::autoscaling::v2::ResourceMetricStatus {
    fn into_optioned(self) -> ResourceMetricStatusAc {
        ResourceMetricStatusAc {
            current: Some(crate::OptionableConvert::into_optioned(self.current)),
            name: Some(self.name),
        }
    }
    fn try_from_optioned(value: ResourceMetricStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            current: crate::OptionableConvert::try_from_optioned(
                value
                    .current
                    .ok_or(crate::Error {
                        missing_field: "current",
                    })?,
            )?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
        })
    }
    fn merge(&mut self, other: ResourceMetricStatusAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.current {
            self.current = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.name {
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::autoscaling::v2::ResourceMetricStatus>
for ResourceMetricStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::autoscaling::v2::ResourceMetricStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::autoscaling::v2::ResourceMetricStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::autoscaling::v2::ResourceMetricStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
