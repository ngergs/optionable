#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DaemonSetSpec is the specification of a daemon set.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DaemonSetSpecAc {
    /// The minimum number of seconds for which a newly created DaemonSet pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,
    /// The number of old history to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. Defaults to 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,
    /// A label query over pods that are managed by the daemon set. Must match in order to be controlled. It must match the pod template's labels. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    /// An object that describes the pod that will be created. The DaemonSet will create exactly one copy of this pod on every node that matches the template's node selector (or on every node if no node selector is specified). The only allowed template.spec.restartPolicy value is "Always". More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#pod-template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<
        <::k8s_openapi027::api::core::v1::PodTemplateSpec as crate::Optionable>::Optioned,
    >,
    /// An update strategy to replace existing DaemonSet pods with new pods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_strategy: Option<
        <::k8s_openapi027::api::apps::v1::DaemonSetUpdateStrategy as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::apps::v1::DaemonSetSpec {
    type Optioned = DaemonSetSpecAc;
}
#[automatically_derived]
impl crate::Optionable for DaemonSetSpecAc {
    type Optioned = DaemonSetSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::apps::v1::DaemonSetSpec {
    fn into_optioned(self) -> DaemonSetSpecAc {
        DaemonSetSpecAc {
            min_ready_seconds: self.min_ready_seconds,
            revision_history_limit: self.revision_history_limit,
            selector: Some(crate::OptionableConvert::into_optioned(self.selector)),
            template: Some(crate::OptionableConvert::into_optioned(self.template)),
            update_strategy: crate::OptionableConvert::into_optioned(
                self.update_strategy,
            ),
        }
    }
    fn try_from_optioned(value: DaemonSetSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            min_ready_seconds: value.min_ready_seconds,
            revision_history_limit: value.revision_history_limit,
            selector: crate::OptionableConvert::try_from_optioned(
                value
                    .selector
                    .ok_or(crate::Error {
                        missing_field: "selector",
                    })?,
            )?,
            template: crate::OptionableConvert::try_from_optioned(
                value
                    .template
                    .ok_or(crate::Error {
                        missing_field: "template",
                    })?,
            )?,
            update_strategy: crate::OptionableConvert::try_from_optioned(
                value.update_strategy,
            )?,
        })
    }
    fn merge(&mut self, other: DaemonSetSpecAc) -> Result<(), crate::Error> {
        if self.min_ready_seconds.is_none() {
            self.min_ready_seconds = crate::OptionableConvert::try_from_optioned(
                other.min_ready_seconds,
            )?;
        } else if let Some(self_value) = self.min_ready_seconds.as_mut()
            && let Some(other_value) = other.min_ready_seconds
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.revision_history_limit.is_none() {
            self.revision_history_limit = crate::OptionableConvert::try_from_optioned(
                other.revision_history_limit,
            )?;
        } else if let Some(self_value) = self.revision_history_limit.as_mut()
            && let Some(other_value) = other.revision_history_limit
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.selector {
            self.selector = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.template {
            self.template = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.update_strategy.is_none() {
            self.update_strategy = crate::OptionableConvert::try_from_optioned(
                other.update_strategy,
            )?;
        } else if let Some(self_value) = self.update_strategy.as_mut()
            && let Some(other_value) = other.update_strategy
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::apps::v1::DaemonSetSpec>
for DaemonSetSpecAc {
    fn from_optionable(value: k8s_openapi027::api::apps::v1::DaemonSetSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::apps::v1::DaemonSetSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apps::v1::DaemonSetSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
