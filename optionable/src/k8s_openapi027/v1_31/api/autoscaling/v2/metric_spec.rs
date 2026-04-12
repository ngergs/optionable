#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// MetricSpec specifies how to scale based on a single metric (only `type` and one other matching field should be set at once).
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MetricSpecAc {
    /// containerResource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing a single container in each pod of the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source. This is an alpha feature and can be enabled by the HPAContainerMetrics feature flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_resource: Option<
        <::k8s_openapi027::api::autoscaling::v2::ContainerResourceMetricSource as crate::Optionable>::Optioned,
    >,
    /// external refers to a global metric that is not associated with any Kubernetes object. It allows autoscaling based on information coming from components running outside of cluster (for example length of queue in cloud messaging service, or QPS from loadbalancer running outside of cluster).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<
        <::k8s_openapi027::api::autoscaling::v2::ExternalMetricSource as crate::Optionable>::Optioned,
    >,
    /// object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<
        <::k8s_openapi027::api::autoscaling::v2::ObjectMetricSource as crate::Optionable>::Optioned,
    >,
    /// pods refers to a metric describing each pod in the current scale target (for example, transactions-processed-per-second).  The values will be averaged together before being compared to the target value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pods: Option<
        <::k8s_openapi027::api::autoscaling::v2::PodsMetricSource as crate::Optionable>::Optioned,
    >,
    /// resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the "pods" source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<
        <::k8s_openapi027::api::autoscaling::v2::ResourceMetricSource as crate::Optionable>::Optioned,
    >,
    /// type is the type of metric source.  It should be one of "ContainerResource", "External", "Object", "Pods" or "Resource", each mapping to a matching field in the object. Note: "ContainerResource" type is available on when the feature-gate HPAContainerMetrics is enabled
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::autoscaling::v2::MetricSpec {
    type Optioned = MetricSpecAc;
}
#[automatically_derived]
impl crate::Optionable for MetricSpecAc {
    type Optioned = MetricSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::autoscaling::v2::MetricSpec {
    fn into_optioned(self) -> MetricSpecAc {
        MetricSpecAc {
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
    fn try_from_optioned(value: MetricSpecAc) -> Result<Self, crate::Error> {
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
    fn merge(&mut self, other: MetricSpecAc) -> Result<(), crate::Error> {
        if self.container_resource.is_none() {
            self.container_resource = crate::OptionableConvert::try_from_optioned(
                other.container_resource,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.container_resource,
                other.container_resource,
            )?;
        }
        if self.external.is_none() {
            self.external = crate::OptionableConvert::try_from_optioned(other.external)?;
        } else {
            crate::OptionableConvert::merge(&mut self.external, other.external)?;
        }
        if self.object.is_none() {
            self.object = crate::OptionableConvert::try_from_optioned(other.object)?;
        } else {
            crate::OptionableConvert::merge(&mut self.object, other.object)?;
        }
        if self.pods.is_none() {
            self.pods = crate::OptionableConvert::try_from_optioned(other.pods)?;
        } else {
            crate::OptionableConvert::merge(&mut self.pods, other.pods)?;
        }
        if self.resource.is_none() {
            self.resource = crate::OptionableConvert::try_from_optioned(other.resource)?;
        } else {
            crate::OptionableConvert::merge(&mut self.resource, other.resource)?;
        }
        if let Some(other_value) = other.type_ {
            self.type_ = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::autoscaling::v2::MetricSpec>
for MetricSpecAc {
    fn from_optionable(value: k8s_openapi027::api::autoscaling::v2::MetricSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::autoscaling::v2::MetricSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::autoscaling::v2::MetricSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
