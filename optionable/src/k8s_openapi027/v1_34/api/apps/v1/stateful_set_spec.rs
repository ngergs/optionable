#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// A StatefulSetSpec is the specification of a StatefulSet.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StatefulSetSpecAc {
    /// Minimum number of seconds for which a newly created pod should be ready without any of its container crashing for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: Option<i32>,
    /// ordinals controls the numbering of replica indices in a StatefulSet. The default ordinals behavior assigns a "0" index to the first replica and increments the index by one for each additional replica requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinals: Option<
        <::k8s_openapi027::api::apps::v1::StatefulSetOrdinals as crate::Optionable>::Optioned,
    >,
    /// persistentVolumeClaimRetentionPolicy describes the lifecycle of persistent volume claims created from volumeClaimTemplates. By default, all persistent volume claims are created as needed and retained until manually deleted. This policy allows the lifecycle to be altered, for example by deleting persistent volume claims when their stateful set is deleted, or when their pod is scaled down.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_volume_claim_retention_policy: Option<
        <::k8s_openapi027::api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy as crate::Optionable>::Optioned,
    >,
    /// podManagementPolicy controls how pods are created during initial scale up, when replacing pods on nodes, or when scaling down. The default policy is `OrderedReady`, where pods are created in increasing order (pod-0, then pod-1, etc) and the controller will wait until each pod is ready before continuing. When scaling down, the pods are removed in the opposite order. The alternative policy is `Parallel` which will create pods in parallel to match the desired scale without waiting, and on scale down will delete all pods at once.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_management_policy: Option<std::string::String>,
    /// replicas is the desired number of replicas of the given Template. These are replicas in the sense that they are instantiations of the same Template, but individual replicas also have a consistent identity. If unspecified, defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// revisionHistoryLimit is the maximum number of revisions that will be maintained in the StatefulSet's revision history. The revision history consists of all revisions not represented by a currently applied StatefulSetSpec version. The default value is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: Option<i32>,
    /// selector is a label query over pods that should match the replica count. It must match the pod template's labels. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    /// serviceName is the name of the service that governs this StatefulSet. This service must exist before the StatefulSet, and is responsible for the network identity of the set. Pods get DNS/hostnames that follow the pattern: pod-specific-string.serviceName.default.svc.cluster.local where "pod-specific-string" is managed by the StatefulSet controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<std::string::String>,
    /// template is the object that describes the pod that will be created if insufficient replicas are detected. Each pod stamped out by the StatefulSet will fulfill this Template, but have a unique identity from the rest of the StatefulSet. Each pod will be named with the format \<statefulsetname\>-\<podindex\>. For example, a pod in a StatefulSet named "web" with index number "3" would be named "web-3". The only allowed template.spec.restartPolicy value is "Always".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<
        <::k8s_openapi027::api::core::v1::PodTemplateSpec as crate::Optionable>::Optioned,
    >,
    /// updateStrategy indicates the StatefulSetUpdateStrategy that will be employed to update Pods in the StatefulSet when a revision is made to Template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_strategy: Option<
        <::k8s_openapi027::api::apps::v1::StatefulSetUpdateStrategy as crate::Optionable>::Optioned,
    >,
    /// volumeClaimTemplates is a list of claims that pods are allowed to reference. The StatefulSet controller is responsible for mapping network identities to claims in a way that maintains the identity of a pod. Every claim in this list must have at least one matching (by name) volumeMount in one container in the template. A claim in this list takes precedence over any volumes in the template, with the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_claim_templates: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::PersistentVolumeClaim as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::apps::v1::StatefulSetSpec {
    type Optioned = StatefulSetSpecAc;
}
#[automatically_derived]
impl crate::Optionable for StatefulSetSpecAc {
    type Optioned = StatefulSetSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::apps::v1::StatefulSetSpec {
    fn into_optioned(self) -> StatefulSetSpecAc {
        StatefulSetSpecAc {
            min_ready_seconds: self.min_ready_seconds,
            ordinals: crate::OptionableConvert::into_optioned(self.ordinals),
            persistent_volume_claim_retention_policy: crate::OptionableConvert::into_optioned(
                self.persistent_volume_claim_retention_policy,
            ),
            pod_management_policy: self.pod_management_policy,
            replicas: self.replicas,
            revision_history_limit: self.revision_history_limit,
            selector: Some(crate::OptionableConvert::into_optioned(self.selector)),
            service_name: self.service_name,
            template: Some(crate::OptionableConvert::into_optioned(self.template)),
            update_strategy: crate::OptionableConvert::into_optioned(
                self.update_strategy,
            ),
            volume_claim_templates: crate::OptionableConvert::into_optioned(
                self.volume_claim_templates,
            ),
        }
    }
    fn try_from_optioned(value: StatefulSetSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            min_ready_seconds: value.min_ready_seconds,
            ordinals: crate::OptionableConvert::try_from_optioned(value.ordinals)?,
            persistent_volume_claim_retention_policy: crate::OptionableConvert::try_from_optioned(
                value.persistent_volume_claim_retention_policy,
            )?,
            pod_management_policy: value.pod_management_policy,
            replicas: value.replicas,
            revision_history_limit: value.revision_history_limit,
            selector: crate::OptionableConvert::try_from_optioned(
                value
                    .selector
                    .ok_or(crate::Error {
                        missing_field: "selector",
                    })?,
            )?,
            service_name: value.service_name,
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
            volume_claim_templates: crate::OptionableConvert::try_from_optioned(
                value.volume_claim_templates,
            )?,
        })
    }
    fn merge(&mut self, other: StatefulSetSpecAc) -> Result<(), crate::Error> {
        if other.min_ready_seconds.is_some() {
            self.min_ready_seconds = other.min_ready_seconds;
        }
        crate::OptionableConvert::merge(&mut self.ordinals, other.ordinals)?;
        crate::OptionableConvert::merge(
            &mut self.persistent_volume_claim_retention_policy,
            other.persistent_volume_claim_retention_policy,
        )?;
        if other.pod_management_policy.is_some() {
            self.pod_management_policy = other.pod_management_policy;
        }
        if other.replicas.is_some() {
            self.replicas = other.replicas;
        }
        if other.revision_history_limit.is_some() {
            self.revision_history_limit = other.revision_history_limit;
        }
        if let Some(other_value) = other.selector {
            crate::OptionableConvert::merge(&mut self.selector, other_value)?;
        }
        if other.service_name.is_some() {
            self.service_name = other.service_name;
        }
        if let Some(other_value) = other.template {
            crate::OptionableConvert::merge(&mut self.template, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.update_strategy,
            other.update_strategy,
        )?;
        crate::OptionableConvert::merge(
            &mut self.volume_claim_templates,
            other.volume_claim_templates,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::apps::v1::StatefulSetSpec>
for StatefulSetSpecAc {
    fn from_optionable(value: k8s_openapi027::api::apps::v1::StatefulSetSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::apps::v1::StatefulSetSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apps::v1::StatefulSetSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
