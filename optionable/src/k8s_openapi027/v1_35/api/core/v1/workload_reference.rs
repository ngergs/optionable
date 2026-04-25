#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// WorkloadReference identifies the Workload object and PodGroup membership that a Pod belongs to. The scheduler uses this information to apply workload-aware scheduling semantics.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkloadReferenceAc {
    /// Name defines the name of the Workload object this Pod belongs to. Workload must be in the same namespace as the Pod. If it doesn't match any existing Workload, the Pod will remain unschedulable until a Workload object is created and observed by the kube-scheduler. It must be a DNS subdomain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// PodGroup is the name of the PodGroup within the Workload that this Pod belongs to. If it doesn't match any existing PodGroup within the Workload, the Pod will remain unschedulable until the Workload object is recreated and observed by the kube-scheduler. It must be a DNS label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_group: Option<std::string::String>,
    /// PodGroupReplicaKey specifies the replica key of the PodGroup to which this Pod belongs. It is used to distinguish pods belonging to different replicas of the same pod group. The pod group policy is applied separately to each replica. When set, it must be a DNS label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_group_replica_key: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::WorkloadReference {
    type Optioned = WorkloadReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for WorkloadReferenceAc {
    type Optioned = WorkloadReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::WorkloadReference {
    fn into_optioned(self) -> WorkloadReferenceAc {
        WorkloadReferenceAc {
            name: Some(self.name),
            pod_group: Some(self.pod_group),
            pod_group_replica_key: self.pod_group_replica_key,
        }
    }
    fn try_from_optioned(value: WorkloadReferenceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            pod_group: value
                .pod_group
                .ok_or(crate::Error {
                    missing_field: "pod_group",
                })?,
            pod_group_replica_key: value.pod_group_replica_key,
        })
    }
    fn merge(&mut self, other: WorkloadReferenceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.pod_group {
            self.pod_group = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.pod_group_replica_key.is_none() {
            self.pod_group_replica_key = crate::OptionableConvert::try_from_optioned(
                other.pod_group_replica_key,
            )?;
        } else if let Some(self_value) = self.pod_group_replica_key.as_mut()
            && let Some(other_value) = other.pod_group_replica_key
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::WorkloadReference>
for WorkloadReferenceAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::WorkloadReference) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::WorkloadReference, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::WorkloadReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for WorkloadReferenceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.name, other.name);
        k8s_openapi027::DeepMerge::merge_from(&mut self.pod_group, other.pod_group);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.pod_group_replica_key,
            other.pod_group_replica_key,
        );
    }
}
