#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ReplicaSetSpec is the specification of a ReplicaSet.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplicaSetSpecAc {
    /// Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,
    /// Replicas is the number of desired replicas. This is a pointer to distinguish between explicit zero and unspecified. Defaults to 1. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller/#what-is-a-replicationcontroller
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// Selector is a label query over pods that should match the replica count. Label keys and values that must match in order to be controlled by this replica set. It must match the pod template's labels. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    /// Template is the object that describes the pod that will be created if insufficient replicas are detected. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#pod-template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<
        <::k8s_openapi027::api::core::v1::PodTemplateSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::apps::v1::ReplicaSetSpec {
    type Optioned = ReplicaSetSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ReplicaSetSpecAc {
    type Optioned = ReplicaSetSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::apps::v1::ReplicaSetSpec {
    fn into_optioned(self) -> ReplicaSetSpecAc {
        ReplicaSetSpecAc {
            min_ready_seconds: self.min_ready_seconds,
            replicas: self.replicas,
            selector: Some(crate::OptionableConvert::into_optioned(self.selector)),
            template: crate::OptionableConvert::into_optioned(self.template),
        }
    }
    fn try_from_optioned(value: ReplicaSetSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            min_ready_seconds: value.min_ready_seconds,
            replicas: value.replicas,
            selector: crate::OptionableConvert::try_from_optioned(
                value
                    .selector
                    .ok_or(crate::Error {
                        missing_field: "selector",
                    })?,
            )?,
            template: crate::OptionableConvert::try_from_optioned(value.template)?,
        })
    }
    fn merge(&mut self, other: ReplicaSetSpecAc) -> Result<(), crate::Error> {
        if self.min_ready_seconds.is_none() {
            self.min_ready_seconds = crate::OptionableConvert::try_from_optioned(
                other.min_ready_seconds,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.min_ready_seconds,
                other.min_ready_seconds,
            )?;
        }
        if self.replicas.is_none() {
            self.replicas = crate::OptionableConvert::try_from_optioned(other.replicas)?;
        } else {
            crate::OptionableConvert::merge(&mut self.replicas, other.replicas)?;
        }
        if let Some(other_value) = other.selector {
            self.selector = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.template.is_none() {
            self.template = crate::OptionableConvert::try_from_optioned(other.template)?;
        } else {
            crate::OptionableConvert::merge(&mut self.template, other.template)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::apps::v1::ReplicaSetSpec>
for ReplicaSetSpecAc {
    fn from_optionable(value: k8s_openapi027::api::apps::v1::ReplicaSetSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::apps::v1::ReplicaSetSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apps::v1::ReplicaSetSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
