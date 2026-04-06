#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ReplicationControllerSpec is the specification of a replication controller.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplicationControllerSpecAc {
    /// Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,
    /// Replicas is the number of desired replicas. This is a pointer to distinguish between explicit zero and unspecified. Defaults to 1. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#what-is-a-replicationcontroller
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// Selector is a label query over pods that should match the Replicas count. If Selector is empty, it is defaulted to the labels present on the Pod template. Label keys and values that must match in order to be controlled by this replication controller, if empty defaulted to labels on Pod template. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
    /// Template is the object that describes the pod that will be created if insufficient replicas are detected. This takes precedence over a TemplateRef. The only allowed template.spec.restartPolicy value is "Always". More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#pod-template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<
        <::k8s_openapi027::api::core::v1::PodTemplateSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ReplicationControllerSpec {
    type Optioned = ReplicationControllerSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ReplicationControllerSpecAc {
    type Optioned = ReplicationControllerSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::ReplicationControllerSpec {
    fn into_optioned(self) -> ReplicationControllerSpecAc {
        ReplicationControllerSpecAc {
            min_ready_seconds: self.min_ready_seconds,
            replicas: self.replicas,
            selector: self.selector,
            template: crate::OptionableConvert::into_optioned(self.template),
        }
    }
    fn try_from_optioned(
        value: ReplicationControllerSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            min_ready_seconds: value.min_ready_seconds,
            replicas: value.replicas,
            selector: value.selector,
            template: crate::OptionableConvert::try_from_optioned(value.template)?,
        })
    }
    fn merge(&mut self, other: ReplicationControllerSpecAc) -> Result<(), crate::Error> {
        self.min_ready_seconds = other.min_ready_seconds;
        self.replicas = other.replicas;
        self.selector = other.selector;
        crate::OptionableConvert::merge(&mut self.template, other.template)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ReplicationControllerSpec>
for ReplicationControllerSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ReplicationControllerSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ReplicationControllerSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ReplicationControllerSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
