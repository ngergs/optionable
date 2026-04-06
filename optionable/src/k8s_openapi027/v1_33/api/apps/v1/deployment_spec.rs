#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DeploymentSpec is the specification of the desired behavior of the Deployment.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeploymentSpecAc {
    /// Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,
    /// Indicates that the deployment is paused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    /// The maximum time in seconds for a deployment to make progress before it is considered to be failed. The deployment controller will continue to process failed deployments and a condition with a ProgressDeadlineExceeded reason will be surfaced in the deployment status. Note that progress will not be estimated during the time a deployment is paused. Defaults to 600s.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_deadline_seconds: Option<i32>,
    /// Number of desired pods. This is a pointer to distinguish between explicit zero and not specified. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// The number of old ReplicaSets to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. Defaults to 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,
    /// Label selector for pods. Existing ReplicaSets whose pods are selected by this will be the ones affected by this deployment. It must match the pod template's labels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    /// The deployment strategy to use to replace existing pods with new ones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<
        <::k8s_openapi027::api::apps::v1::DeploymentStrategy as crate::Optionable>::Optioned,
    >,
    /// Template describes the pods that will be created. The only allowed template.spec.restartPolicy value is "Always".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<
        <::k8s_openapi027::api::core::v1::PodTemplateSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::apps::v1::DeploymentSpec {
    type Optioned = DeploymentSpecAc;
}
#[automatically_derived]
impl crate::Optionable for DeploymentSpecAc {
    type Optioned = DeploymentSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::apps::v1::DeploymentSpec {
    fn into_optioned(self) -> DeploymentSpecAc {
        DeploymentSpecAc {
            min_ready_seconds: self.min_ready_seconds,
            paused: self.paused,
            progress_deadline_seconds: self.progress_deadline_seconds,
            replicas: self.replicas,
            revision_history_limit: self.revision_history_limit,
            selector: Some(crate::OptionableConvert::into_optioned(self.selector)),
            strategy: crate::OptionableConvert::into_optioned(self.strategy),
            template: Some(crate::OptionableConvert::into_optioned(self.template)),
        }
    }
    fn try_from_optioned(value: DeploymentSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            min_ready_seconds: value.min_ready_seconds,
            paused: value.paused,
            progress_deadline_seconds: value.progress_deadline_seconds,
            replicas: value.replicas,
            revision_history_limit: value.revision_history_limit,
            selector: crate::OptionableConvert::try_from_optioned(
                value
                    .selector
                    .ok_or(crate::Error {
                        missing_field: "selector",
                    })?,
            )?,
            strategy: crate::OptionableConvert::try_from_optioned(value.strategy)?,
            template: crate::OptionableConvert::try_from_optioned(
                value
                    .template
                    .ok_or(crate::Error {
                        missing_field: "template",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: DeploymentSpecAc) -> Result<(), crate::Error> {
        self.min_ready_seconds = other.min_ready_seconds;
        self.paused = other.paused;
        self.progress_deadline_seconds = other.progress_deadline_seconds;
        self.replicas = other.replicas;
        self.revision_history_limit = other.revision_history_limit;
        if let Some(other_value) = other.selector {
            crate::OptionableConvert::merge(&mut self.selector, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.strategy, other.strategy)?;
        if let Some(other_value) = other.template {
            crate::OptionableConvert::merge(&mut self.template, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::apps::v1::DeploymentSpec>
for DeploymentSpecAc {
    fn from_optionable(value: k8s_openapi027::api::apps::v1::DeploymentSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::apps::v1::DeploymentSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apps::v1::DeploymentSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
