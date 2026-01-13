#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: <Option<i64> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affinity: <Option<
        ::k8s_openapi027::api::core::v1::Affinity,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automount_service_account_token: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<
        <std::vec::Vec<
            ::k8s_openapi027::api::core::v1::Container,
        > as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config: <Option<
        ::k8s_openapi027::api::core::v1::PodDNSConfig,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_service_links: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_containers: <Option<
        std::vec::Vec<::k8s_openapi027::api::core::v1::EphemeralContainer>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_aliases: <Option<
        std::vec::Vec<::k8s_openapi027::api::core::v1::HostAlias>,
    > as crate::Optionable>::Optioned,
    #[serde(rename = "hostIPC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_ipc: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_network: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(rename = "hostPID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_pid: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_users: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_secrets: <Option<
        std::vec::Vec<::k8s_openapi027::api::core::v1::LocalObjectReference>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_containers: <Option<
        std::vec::Vec<::k8s_openapi027::api::core::v1::Container>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_selector: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: <Option<
        ::k8s_openapi027::api::core::v1::PodOS,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overhead: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi027::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preemption_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_class_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_gates: <Option<
        std::vec::Vec<::k8s_openapi027::api::core::v1::PodReadinessGate>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claims: <Option<
        std::vec::Vec<::k8s_openapi027::api::core::v1::PodResourceClaim>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: <Option<
        ::k8s_openapi027::api::core::v1::ResourceRequirements,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_class_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_gates: <Option<
        std::vec::Vec<::k8s_openapi027::api::core::v1::PodSchedulingGate>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context: <Option<
        ::k8s_openapi027::api::core::v1::PodSecurityContext,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(rename = "setHostnameAsFQDN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_hostname_as_fqdn: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_process_namespace: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: <Option<i64> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerations: <Option<
        std::vec::Vec<::k8s_openapi027::api::core::v1::Toleration>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topology_spread_constraints: <Option<
        std::vec::Vec<::k8s_openapi027::api::core::v1::TopologySpreadConstraint>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: <Option<
        std::vec::Vec<::k8s_openapi027::api::core::v1::Volume>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PodSpec {
    type Optioned = PodSpecAc;
}
#[automatically_derived]
impl crate::Optionable for PodSpecAc {
    type Optioned = PodSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::PodSpec {
    fn into_optioned(self) -> PodSpecAc {
        PodSpecAc {
            active_deadline_seconds: crate::OptionableConvert::into_optioned(
                self.active_deadline_seconds,
            ),
            affinity: crate::OptionableConvert::into_optioned(self.affinity),
            automount_service_account_token: crate::OptionableConvert::into_optioned(
                self.automount_service_account_token,
            ),
            containers: Some(crate::OptionableConvert::into_optioned(self.containers)),
            dns_config: crate::OptionableConvert::into_optioned(self.dns_config),
            dns_policy: crate::OptionableConvert::into_optioned(self.dns_policy),
            enable_service_links: crate::OptionableConvert::into_optioned(
                self.enable_service_links,
            ),
            ephemeral_containers: crate::OptionableConvert::into_optioned(
                self.ephemeral_containers,
            ),
            host_aliases: crate::OptionableConvert::into_optioned(self.host_aliases),
            host_ipc: crate::OptionableConvert::into_optioned(self.host_ipc),
            host_network: crate::OptionableConvert::into_optioned(self.host_network),
            host_pid: crate::OptionableConvert::into_optioned(self.host_pid),
            host_users: crate::OptionableConvert::into_optioned(self.host_users),
            hostname: crate::OptionableConvert::into_optioned(self.hostname),
            image_pull_secrets: crate::OptionableConvert::into_optioned(
                self.image_pull_secrets,
            ),
            init_containers: crate::OptionableConvert::into_optioned(
                self.init_containers,
            ),
            node_name: crate::OptionableConvert::into_optioned(self.node_name),
            node_selector: crate::OptionableConvert::into_optioned(self.node_selector),
            os: crate::OptionableConvert::into_optioned(self.os),
            overhead: crate::OptionableConvert::into_optioned(self.overhead),
            preemption_policy: crate::OptionableConvert::into_optioned(
                self.preemption_policy,
            ),
            priority: crate::OptionableConvert::into_optioned(self.priority),
            priority_class_name: crate::OptionableConvert::into_optioned(
                self.priority_class_name,
            ),
            readiness_gates: crate::OptionableConvert::into_optioned(
                self.readiness_gates,
            ),
            resource_claims: crate::OptionableConvert::into_optioned(
                self.resource_claims,
            ),
            resources: crate::OptionableConvert::into_optioned(self.resources),
            restart_policy: crate::OptionableConvert::into_optioned(self.restart_policy),
            runtime_class_name: crate::OptionableConvert::into_optioned(
                self.runtime_class_name,
            ),
            scheduler_name: crate::OptionableConvert::into_optioned(self.scheduler_name),
            scheduling_gates: crate::OptionableConvert::into_optioned(
                self.scheduling_gates,
            ),
            security_context: crate::OptionableConvert::into_optioned(
                self.security_context,
            ),
            service_account: crate::OptionableConvert::into_optioned(
                self.service_account,
            ),
            service_account_name: crate::OptionableConvert::into_optioned(
                self.service_account_name,
            ),
            set_hostname_as_fqdn: crate::OptionableConvert::into_optioned(
                self.set_hostname_as_fqdn,
            ),
            share_process_namespace: crate::OptionableConvert::into_optioned(
                self.share_process_namespace,
            ),
            subdomain: crate::OptionableConvert::into_optioned(self.subdomain),
            termination_grace_period_seconds: crate::OptionableConvert::into_optioned(
                self.termination_grace_period_seconds,
            ),
            tolerations: crate::OptionableConvert::into_optioned(self.tolerations),
            topology_spread_constraints: crate::OptionableConvert::into_optioned(
                self.topology_spread_constraints,
            ),
            volumes: crate::OptionableConvert::into_optioned(self.volumes),
        }
    }
    fn try_from_optioned(value: PodSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            active_deadline_seconds: crate::OptionableConvert::try_from_optioned(
                value.active_deadline_seconds,
            )?,
            affinity: crate::OptionableConvert::try_from_optioned(value.affinity)?,
            automount_service_account_token: crate::OptionableConvert::try_from_optioned(
                value.automount_service_account_token,
            )?,
            containers: crate::OptionableConvert::try_from_optioned(
                value
                    .containers
                    .ok_or(crate::Error {
                        missing_field: "containers",
                    })?,
            )?,
            dns_config: crate::OptionableConvert::try_from_optioned(value.dns_config)?,
            dns_policy: crate::OptionableConvert::try_from_optioned(value.dns_policy)?,
            enable_service_links: crate::OptionableConvert::try_from_optioned(
                value.enable_service_links,
            )?,
            ephemeral_containers: crate::OptionableConvert::try_from_optioned(
                value.ephemeral_containers,
            )?,
            host_aliases: crate::OptionableConvert::try_from_optioned(
                value.host_aliases,
            )?,
            host_ipc: crate::OptionableConvert::try_from_optioned(value.host_ipc)?,
            host_network: crate::OptionableConvert::try_from_optioned(
                value.host_network,
            )?,
            host_pid: crate::OptionableConvert::try_from_optioned(value.host_pid)?,
            host_users: crate::OptionableConvert::try_from_optioned(value.host_users)?,
            hostname: crate::OptionableConvert::try_from_optioned(value.hostname)?,
            image_pull_secrets: crate::OptionableConvert::try_from_optioned(
                value.image_pull_secrets,
            )?,
            init_containers: crate::OptionableConvert::try_from_optioned(
                value.init_containers,
            )?,
            node_name: crate::OptionableConvert::try_from_optioned(value.node_name)?,
            node_selector: crate::OptionableConvert::try_from_optioned(
                value.node_selector,
            )?,
            os: crate::OptionableConvert::try_from_optioned(value.os)?,
            overhead: crate::OptionableConvert::try_from_optioned(value.overhead)?,
            preemption_policy: crate::OptionableConvert::try_from_optioned(
                value.preemption_policy,
            )?,
            priority: crate::OptionableConvert::try_from_optioned(value.priority)?,
            priority_class_name: crate::OptionableConvert::try_from_optioned(
                value.priority_class_name,
            )?,
            readiness_gates: crate::OptionableConvert::try_from_optioned(
                value.readiness_gates,
            )?,
            resource_claims: crate::OptionableConvert::try_from_optioned(
                value.resource_claims,
            )?,
            resources: crate::OptionableConvert::try_from_optioned(value.resources)?,
            restart_policy: crate::OptionableConvert::try_from_optioned(
                value.restart_policy,
            )?,
            runtime_class_name: crate::OptionableConvert::try_from_optioned(
                value.runtime_class_name,
            )?,
            scheduler_name: crate::OptionableConvert::try_from_optioned(
                value.scheduler_name,
            )?,
            scheduling_gates: crate::OptionableConvert::try_from_optioned(
                value.scheduling_gates,
            )?,
            security_context: crate::OptionableConvert::try_from_optioned(
                value.security_context,
            )?,
            service_account: crate::OptionableConvert::try_from_optioned(
                value.service_account,
            )?,
            service_account_name: crate::OptionableConvert::try_from_optioned(
                value.service_account_name,
            )?,
            set_hostname_as_fqdn: crate::OptionableConvert::try_from_optioned(
                value.set_hostname_as_fqdn,
            )?,
            share_process_namespace: crate::OptionableConvert::try_from_optioned(
                value.share_process_namespace,
            )?,
            subdomain: crate::OptionableConvert::try_from_optioned(value.subdomain)?,
            termination_grace_period_seconds: crate::OptionableConvert::try_from_optioned(
                value.termination_grace_period_seconds,
            )?,
            tolerations: crate::OptionableConvert::try_from_optioned(value.tolerations)?,
            topology_spread_constraints: crate::OptionableConvert::try_from_optioned(
                value.topology_spread_constraints,
            )?,
            volumes: crate::OptionableConvert::try_from_optioned(value.volumes)?,
        })
    }
    fn merge(&mut self, other: PodSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.active_deadline_seconds,
            other.active_deadline_seconds,
        )?;
        crate::OptionableConvert::merge(&mut self.affinity, other.affinity)?;
        crate::OptionableConvert::merge(
            &mut self.automount_service_account_token,
            other.automount_service_account_token,
        )?;
        if let Some(other_value) = other.containers {
            crate::OptionableConvert::merge(&mut self.containers, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.dns_config, other.dns_config)?;
        crate::OptionableConvert::merge(&mut self.dns_policy, other.dns_policy)?;
        crate::OptionableConvert::merge(
            &mut self.enable_service_links,
            other.enable_service_links,
        )?;
        crate::OptionableConvert::merge(
            &mut self.ephemeral_containers,
            other.ephemeral_containers,
        )?;
        crate::OptionableConvert::merge(&mut self.host_aliases, other.host_aliases)?;
        crate::OptionableConvert::merge(&mut self.host_ipc, other.host_ipc)?;
        crate::OptionableConvert::merge(&mut self.host_network, other.host_network)?;
        crate::OptionableConvert::merge(&mut self.host_pid, other.host_pid)?;
        crate::OptionableConvert::merge(&mut self.host_users, other.host_users)?;
        crate::OptionableConvert::merge(&mut self.hostname, other.hostname)?;
        crate::OptionableConvert::merge(
            &mut self.image_pull_secrets,
            other.image_pull_secrets,
        )?;
        crate::OptionableConvert::merge(
            &mut self.init_containers,
            other.init_containers,
        )?;
        crate::OptionableConvert::merge(&mut self.node_name, other.node_name)?;
        crate::OptionableConvert::merge(&mut self.node_selector, other.node_selector)?;
        crate::OptionableConvert::merge(&mut self.os, other.os)?;
        crate::OptionableConvert::merge(&mut self.overhead, other.overhead)?;
        crate::OptionableConvert::merge(
            &mut self.preemption_policy,
            other.preemption_policy,
        )?;
        crate::OptionableConvert::merge(&mut self.priority, other.priority)?;
        crate::OptionableConvert::merge(
            &mut self.priority_class_name,
            other.priority_class_name,
        )?;
        crate::OptionableConvert::merge(
            &mut self.readiness_gates,
            other.readiness_gates,
        )?;
        crate::OptionableConvert::merge(
            &mut self.resource_claims,
            other.resource_claims,
        )?;
        crate::OptionableConvert::merge(&mut self.resources, other.resources)?;
        crate::OptionableConvert::merge(&mut self.restart_policy, other.restart_policy)?;
        crate::OptionableConvert::merge(
            &mut self.runtime_class_name,
            other.runtime_class_name,
        )?;
        crate::OptionableConvert::merge(&mut self.scheduler_name, other.scheduler_name)?;
        crate::OptionableConvert::merge(
            &mut self.scheduling_gates,
            other.scheduling_gates,
        )?;
        crate::OptionableConvert::merge(
            &mut self.security_context,
            other.security_context,
        )?;
        crate::OptionableConvert::merge(
            &mut self.service_account,
            other.service_account,
        )?;
        crate::OptionableConvert::merge(
            &mut self.service_account_name,
            other.service_account_name,
        )?;
        crate::OptionableConvert::merge(
            &mut self.set_hostname_as_fqdn,
            other.set_hostname_as_fqdn,
        )?;
        crate::OptionableConvert::merge(
            &mut self.share_process_namespace,
            other.share_process_namespace,
        )?;
        crate::OptionableConvert::merge(&mut self.subdomain, other.subdomain)?;
        crate::OptionableConvert::merge(
            &mut self.termination_grace_period_seconds,
            other.termination_grace_period_seconds,
        )?;
        crate::OptionableConvert::merge(&mut self.tolerations, other.tolerations)?;
        crate::OptionableConvert::merge(
            &mut self.topology_spread_constraints,
            other.topology_spread_constraints,
        )?;
        crate::OptionableConvert::merge(&mut self.volumes, other.volumes)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PodSpec> for PodSpecAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::PodSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PodSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PodSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
