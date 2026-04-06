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
    pub active_deadline_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affinity: Option<
        <::k8s_openapi027::api::core::v1::Affinity as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automount_service_account_token: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::Container as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<
        <::k8s_openapi027::api::core::v1::PodDNSConfig as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_policy: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_service_links: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_containers: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::EphemeralContainer as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_aliases: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::HostAlias as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(rename = "hostIPC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_ipc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_network: Option<bool>,
    #[serde(rename = "hostPID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_pid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_users: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname_override: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_secrets: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::LocalObjectReference as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_containers: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::Container as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<
        <::k8s_openapi027::api::core::v1::PodOS as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overhead: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preemption_policy: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_class_name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_gates: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::PodReadinessGate as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claims: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::PodResourceClaim as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<
        <::k8s_openapi027::api::core::v1::ResourceRequirements as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_class_name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler_name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_gates: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::PodSchedulingGate as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context: Option<
        <::k8s_openapi027::api::core::v1::PodSecurityContext as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_name: Option<std::string::String>,
    #[serde(rename = "setHostnameAsFQDN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_hostname_as_fqdn: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_process_namespace: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::Toleration as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topology_spread_constraints: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::TopologySpreadConstraint as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::Volume as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_ref: Option<
        <::k8s_openapi027::api::core::v1::WorkloadReference as crate::Optionable>::Optioned,
    >,
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
            active_deadline_seconds: self.active_deadline_seconds,
            affinity: crate::OptionableConvert::into_optioned(self.affinity),
            automount_service_account_token: self.automount_service_account_token,
            containers: Some(crate::OptionableConvert::into_optioned(self.containers)),
            dns_config: crate::OptionableConvert::into_optioned(self.dns_config),
            dns_policy: self.dns_policy,
            enable_service_links: self.enable_service_links,
            ephemeral_containers: crate::OptionableConvert::into_optioned(
                self.ephemeral_containers,
            ),
            host_aliases: crate::OptionableConvert::into_optioned(self.host_aliases),
            host_ipc: self.host_ipc,
            host_network: self.host_network,
            host_pid: self.host_pid,
            host_users: self.host_users,
            hostname: self.hostname,
            hostname_override: self.hostname_override,
            image_pull_secrets: crate::OptionableConvert::into_optioned(
                self.image_pull_secrets,
            ),
            init_containers: crate::OptionableConvert::into_optioned(
                self.init_containers,
            ),
            node_name: self.node_name,
            node_selector: self.node_selector,
            os: crate::OptionableConvert::into_optioned(self.os),
            overhead: crate::OptionableConvert::into_optioned(self.overhead),
            preemption_policy: self.preemption_policy,
            priority: self.priority,
            priority_class_name: self.priority_class_name,
            readiness_gates: crate::OptionableConvert::into_optioned(
                self.readiness_gates,
            ),
            resource_claims: crate::OptionableConvert::into_optioned(
                self.resource_claims,
            ),
            resources: crate::OptionableConvert::into_optioned(self.resources),
            restart_policy: self.restart_policy,
            runtime_class_name: self.runtime_class_name,
            scheduler_name: self.scheduler_name,
            scheduling_gates: crate::OptionableConvert::into_optioned(
                self.scheduling_gates,
            ),
            security_context: crate::OptionableConvert::into_optioned(
                self.security_context,
            ),
            service_account: self.service_account,
            service_account_name: self.service_account_name,
            set_hostname_as_fqdn: self.set_hostname_as_fqdn,
            share_process_namespace: self.share_process_namespace,
            subdomain: self.subdomain,
            termination_grace_period_seconds: self.termination_grace_period_seconds,
            tolerations: crate::OptionableConvert::into_optioned(self.tolerations),
            topology_spread_constraints: crate::OptionableConvert::into_optioned(
                self.topology_spread_constraints,
            ),
            volumes: crate::OptionableConvert::into_optioned(self.volumes),
            workload_ref: crate::OptionableConvert::into_optioned(self.workload_ref),
        }
    }
    fn try_from_optioned(value: PodSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            active_deadline_seconds: value.active_deadline_seconds,
            affinity: crate::OptionableConvert::try_from_optioned(value.affinity)?,
            automount_service_account_token: value.automount_service_account_token,
            containers: crate::OptionableConvert::try_from_optioned(
                value
                    .containers
                    .ok_or(crate::Error {
                        missing_field: "containers",
                    })?,
            )?,
            dns_config: crate::OptionableConvert::try_from_optioned(value.dns_config)?,
            dns_policy: value.dns_policy,
            enable_service_links: value.enable_service_links,
            ephemeral_containers: crate::OptionableConvert::try_from_optioned(
                value.ephemeral_containers,
            )?,
            host_aliases: crate::OptionableConvert::try_from_optioned(
                value.host_aliases,
            )?,
            host_ipc: value.host_ipc,
            host_network: value.host_network,
            host_pid: value.host_pid,
            host_users: value.host_users,
            hostname: value.hostname,
            hostname_override: value.hostname_override,
            image_pull_secrets: crate::OptionableConvert::try_from_optioned(
                value.image_pull_secrets,
            )?,
            init_containers: crate::OptionableConvert::try_from_optioned(
                value.init_containers,
            )?,
            node_name: value.node_name,
            node_selector: value.node_selector,
            os: crate::OptionableConvert::try_from_optioned(value.os)?,
            overhead: crate::OptionableConvert::try_from_optioned(value.overhead)?,
            preemption_policy: value.preemption_policy,
            priority: value.priority,
            priority_class_name: value.priority_class_name,
            readiness_gates: crate::OptionableConvert::try_from_optioned(
                value.readiness_gates,
            )?,
            resource_claims: crate::OptionableConvert::try_from_optioned(
                value.resource_claims,
            )?,
            resources: crate::OptionableConvert::try_from_optioned(value.resources)?,
            restart_policy: value.restart_policy,
            runtime_class_name: value.runtime_class_name,
            scheduler_name: value.scheduler_name,
            scheduling_gates: crate::OptionableConvert::try_from_optioned(
                value.scheduling_gates,
            )?,
            security_context: crate::OptionableConvert::try_from_optioned(
                value.security_context,
            )?,
            service_account: value.service_account,
            service_account_name: value.service_account_name,
            set_hostname_as_fqdn: value.set_hostname_as_fqdn,
            share_process_namespace: value.share_process_namespace,
            subdomain: value.subdomain,
            termination_grace_period_seconds: value.termination_grace_period_seconds,
            tolerations: crate::OptionableConvert::try_from_optioned(value.tolerations)?,
            topology_spread_constraints: crate::OptionableConvert::try_from_optioned(
                value.topology_spread_constraints,
            )?,
            volumes: crate::OptionableConvert::try_from_optioned(value.volumes)?,
            workload_ref: crate::OptionableConvert::try_from_optioned(
                value.workload_ref,
            )?,
        })
    }
    fn merge(&mut self, other: PodSpecAc) -> Result<(), crate::Error> {
        self.active_deadline_seconds = other.active_deadline_seconds;
        crate::OptionableConvert::merge(&mut self.affinity, other.affinity)?;
        self.automount_service_account_token = other.automount_service_account_token;
        if let Some(other_value) = other.containers {
            crate::OptionableConvert::merge(&mut self.containers, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.dns_config, other.dns_config)?;
        self.dns_policy = other.dns_policy;
        self.enable_service_links = other.enable_service_links;
        crate::OptionableConvert::merge(
            &mut self.ephemeral_containers,
            other.ephemeral_containers,
        )?;
        crate::OptionableConvert::merge(&mut self.host_aliases, other.host_aliases)?;
        self.host_ipc = other.host_ipc;
        self.host_network = other.host_network;
        self.host_pid = other.host_pid;
        self.host_users = other.host_users;
        self.hostname = other.hostname;
        self.hostname_override = other.hostname_override;
        crate::OptionableConvert::merge(
            &mut self.image_pull_secrets,
            other.image_pull_secrets,
        )?;
        crate::OptionableConvert::merge(
            &mut self.init_containers,
            other.init_containers,
        )?;
        self.node_name = other.node_name;
        self.node_selector = other.node_selector;
        crate::OptionableConvert::merge(&mut self.os, other.os)?;
        crate::OptionableConvert::merge(&mut self.overhead, other.overhead)?;
        self.preemption_policy = other.preemption_policy;
        self.priority = other.priority;
        self.priority_class_name = other.priority_class_name;
        crate::OptionableConvert::merge(
            &mut self.readiness_gates,
            other.readiness_gates,
        )?;
        crate::OptionableConvert::merge(
            &mut self.resource_claims,
            other.resource_claims,
        )?;
        crate::OptionableConvert::merge(&mut self.resources, other.resources)?;
        self.restart_policy = other.restart_policy;
        self.runtime_class_name = other.runtime_class_name;
        self.scheduler_name = other.scheduler_name;
        crate::OptionableConvert::merge(
            &mut self.scheduling_gates,
            other.scheduling_gates,
        )?;
        crate::OptionableConvert::merge(
            &mut self.security_context,
            other.security_context,
        )?;
        self.service_account = other.service_account;
        self.service_account_name = other.service_account_name;
        self.set_hostname_as_fqdn = other.set_hostname_as_fqdn;
        self.share_process_namespace = other.share_process_namespace;
        self.subdomain = other.subdomain;
        self.termination_grace_period_seconds = other.termination_grace_period_seconds;
        crate::OptionableConvert::merge(&mut self.tolerations, other.tolerations)?;
        crate::OptionableConvert::merge(
            &mut self.topology_spread_constraints,
            other.topology_spread_constraints,
        )?;
        crate::OptionableConvert::merge(&mut self.volumes, other.volumes)?;
        crate::OptionableConvert::merge(&mut self.workload_ref, other.workload_ref)?;
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
