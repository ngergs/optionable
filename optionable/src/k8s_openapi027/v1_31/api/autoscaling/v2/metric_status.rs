#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// MetricStatus describes the last-read state of a single metric.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MetricStatusAc {
    /// container resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing a single container in each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_resource: Option<
        <::k8s_openapi027::api::autoscaling::v2::ContainerResourceMetricStatus as crate::Optionable>::Optioned,
    >,
    /// external refers to a global metric that is not associated with any Kubernetes object. It allows autoscaling based on information coming from components running outside of cluster (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<
        <::k8s_openapi027::api::autoscaling::v2::ExternalMetricStatus as crate::Optionable>::Optioned,
    >,
    /// object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<
        <::k8s_openapi027::api::autoscaling::v2::ObjectMetricStatus as crate::Optionable>::Optioned,
    >,
    /// pods refers to a metric describing each pod in the current scale target (for example, transactions-processed-per-second).  The values will be averaged together before being compared to the target value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pods: Option<
        <::k8s_openapi027::api::autoscaling::v2::PodsMetricStatus as crate::Optionable>::Optioned,
    >,
    /// resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<
        <::k8s_openapi027::api::autoscaling::v2::ResourceMetricStatus as crate::Optionable>::Optioned,
    >,
    /// type is the type of metric source.  It will be one of "ContainerResource", "External", "Object", "Pods" or "Resource", each corresponds to a matching field in the object. Note: "ContainerResource" type is available on when the feature-gate HPAContainerMetrics is enabled
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::autoscaling::v2::MetricStatus {
    type Optioned = MetricStatusAc;
}
#[automatically_derived]
impl crate::Optionable for MetricStatusAc {
    type Optioned = MetricStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::autoscaling::v2::MetricStatus {
    fn into_optioned(self) -> MetricStatusAc {
        MetricStatusAc {
            container_resource: crate::OptionableConvert::into_optioned(
                self.container_resource,
            ),
            external: crate::OptionableConvert::into_optioned(self.external),
            object: crate::OptionableConvert::into_optioned(self.object),
            pods: crate::OptionableConvert::into_optioned(self.pods),
            resource: crate::OptionableConvert::into_optioned(self.resource),
            type_: Some(self.type_),
        }
    }
    fn try_from_optioned(value: MetricStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            container_resource: crate::OptionableConvert::try_from_optioned(
                value.container_resource,
            )?,
            external: crate::OptionableConvert::try_from_optioned(value.external)?,
            object: crate::OptionableConvert::try_from_optioned(value.object)?,
            pods: crate::OptionableConvert::try_from_optioned(value.pods)?,
            resource: crate::OptionableConvert::try_from_optioned(value.resource)?,
            type_: value
                .type_
                .ok_or(crate::Error {
                    missing_field: "type_",
                })?,
        })
    }
    fn merge(&mut self, other: MetricStatusAc) -> Result<(), crate::Error> {
        if self.container_resource.is_none() {
            self.container_resource = other.container_resource;
        }
        if let Some(other_value) = other.container_resource {
            crate::OptionableConvert::merge(&mut self.container_resource, other_value)?;
        }
        if self.external.is_none() {
            self.external = other.external;
        }
        if let Some(other_value) = other.external {
            crate::OptionableConvert::merge(&mut self.external, other_value)?;
        }
        if self.object.is_none() {
            self.object = other.object;
        }
        if let Some(other_value) = other.object {
            crate::OptionableConvert::merge(&mut self.object, other_value)?;
        }
        if self.pods.is_none() {
            self.pods = other.pods;
        }
        if let Some(other_value) = other.pods {
            crate::OptionableConvert::merge(&mut self.pods, other_value)?;
        }
        if self.resource.is_none() {
            self.resource = other.resource;
        }
        if let Some(other_value) = other.resource {
            crate::OptionableConvert::merge(&mut self.resource, other_value)?;
        }
        if let Some(other_value) = other.type_ {
            self.type_ = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::autoscaling::v2::MetricStatus>
for MetricStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::autoscaling::v2::MetricStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::autoscaling::v2::MetricStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::autoscaling::v2::MetricStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
