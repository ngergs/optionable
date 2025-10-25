pub struct PodSpecOpt {
    pub active_deadline_seconds: <Option<i64> as crate::Optionable>::Optioned,
    pub affinity: <Option<
        ::k8s_openapi::api::core::v1::Affinity,
    > as crate::Optionable>::Optioned,
    pub automount_service_account_token: <Option<bool> as crate::Optionable>::Optioned,
    pub containers: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::core::v1::Container,
        > as crate::Optionable>::Optioned,
    >,
    pub dns_config: <Option<
        ::k8s_openapi::api::core::v1::PodDNSConfig,
    > as crate::Optionable>::Optioned,
    pub dns_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub enable_service_links: <Option<bool> as crate::Optionable>::Optioned,
    pub ephemeral_containers: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::EphemeralContainer>,
    > as crate::Optionable>::Optioned,
    pub host_aliases: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::HostAlias>,
    > as crate::Optionable>::Optioned,
    pub host_ipc: <Option<bool> as crate::Optionable>::Optioned,
    pub host_network: <Option<bool> as crate::Optionable>::Optioned,
    pub host_pid: <Option<bool> as crate::Optionable>::Optioned,
    pub host_users: <Option<bool> as crate::Optionable>::Optioned,
    pub hostname: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub hostname_override: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub image_pull_secrets: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::LocalObjectReference>,
    > as crate::Optionable>::Optioned,
    pub init_containers: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::Container>,
    > as crate::Optionable>::Optioned,
    pub node_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub node_selector: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    pub os: <Option<::k8s_openapi::api::core::v1::PodOS> as crate::Optionable>::Optioned,
    pub overhead: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    pub preemption_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub priority: <Option<i32> as crate::Optionable>::Optioned,
    pub priority_class_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub readiness_gates: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::PodReadinessGate>,
    > as crate::Optionable>::Optioned,
    pub resource_claims: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::PodResourceClaim>,
    > as crate::Optionable>::Optioned,
    pub resources: <Option<
        ::k8s_openapi::api::core::v1::ResourceRequirements,
    > as crate::Optionable>::Optioned,
    pub restart_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub runtime_class_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub scheduler_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub scheduling_gates: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::PodSchedulingGate>,
    > as crate::Optionable>::Optioned,
    pub security_context: <Option<
        ::k8s_openapi::api::core::v1::PodSecurityContext,
    > as crate::Optionable>::Optioned,
    pub service_account: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub service_account_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub set_hostname_as_fqdn: <Option<bool> as crate::Optionable>::Optioned,
    pub share_process_namespace: <Option<bool> as crate::Optionable>::Optioned,
    pub subdomain: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub termination_grace_period_seconds: <Option<i64> as crate::Optionable>::Optioned,
    pub tolerations: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::Toleration>,
    > as crate::Optionable>::Optioned,
    pub topology_spread_constraints: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::TopologySpreadConstraint>,
    > as crate::Optionable>::Optioned,
    pub volumes: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::Volume>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PodSpec {
    type Optioned = PodSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for PodSpecOpt {
    type Optioned = PodSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PodSpec {
    fn into_optioned(self) -> PodSpecOpt {
        PodSpecOpt {
            active_deadline_seconds: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(self.active_deadline_seconds),
            affinity: <Option<
                ::k8s_openapi::api::core::v1::Affinity,
            > as crate::OptionableConvert>::into_optioned(self.affinity),
            automount_service_account_token: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(
                self.automount_service_account_token,
            ),
            containers: Some(
                <std::vec::Vec<
                    ::k8s_openapi::api::core::v1::Container,
                > as crate::OptionableConvert>::into_optioned(self.containers),
            ),
            dns_config: <Option<
                ::k8s_openapi::api::core::v1::PodDNSConfig,
            > as crate::OptionableConvert>::into_optioned(self.dns_config),
            dns_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.dns_policy),
            enable_service_links: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.enable_service_links),
            ephemeral_containers: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::EphemeralContainer>,
            > as crate::OptionableConvert>::into_optioned(self.ephemeral_containers),
            host_aliases: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::HostAlias>,
            > as crate::OptionableConvert>::into_optioned(self.host_aliases),
            host_ipc: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.host_ipc),
            host_network: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.host_network),
            host_pid: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.host_pid),
            host_users: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.host_users),
            hostname: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.hostname),
            hostname_override: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.hostname_override),
            image_pull_secrets: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::LocalObjectReference>,
            > as crate::OptionableConvert>::into_optioned(self.image_pull_secrets),
            init_containers: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::Container>,
            > as crate::OptionableConvert>::into_optioned(self.init_containers),
            node_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.node_name),
            node_selector: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.node_selector),
            os: <Option<
                ::k8s_openapi::api::core::v1::PodOS,
            > as crate::OptionableConvert>::into_optioned(self.os),
            overhead: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::into_optioned(self.overhead),
            preemption_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.preemption_policy),
            priority: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.priority),
            priority_class_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.priority_class_name),
            readiness_gates: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::PodReadinessGate>,
            > as crate::OptionableConvert>::into_optioned(self.readiness_gates),
            resource_claims: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::PodResourceClaim>,
            > as crate::OptionableConvert>::into_optioned(self.resource_claims),
            resources: <Option<
                ::k8s_openapi::api::core::v1::ResourceRequirements,
            > as crate::OptionableConvert>::into_optioned(self.resources),
            restart_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.restart_policy),
            runtime_class_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.runtime_class_name),
            scheduler_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.scheduler_name),
            scheduling_gates: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::PodSchedulingGate>,
            > as crate::OptionableConvert>::into_optioned(self.scheduling_gates),
            security_context: <Option<
                ::k8s_openapi::api::core::v1::PodSecurityContext,
            > as crate::OptionableConvert>::into_optioned(self.security_context),
            service_account: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.service_account),
            service_account_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.service_account_name),
            set_hostname_as_fqdn: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.set_hostname_as_fqdn),
            share_process_namespace: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.share_process_namespace),
            subdomain: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.subdomain),
            termination_grace_period_seconds: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(
                self.termination_grace_period_seconds,
            ),
            tolerations: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::Toleration>,
            > as crate::OptionableConvert>::into_optioned(self.tolerations),
            topology_spread_constraints: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::TopologySpreadConstraint>,
            > as crate::OptionableConvert>::into_optioned(
                self.topology_spread_constraints,
            ),
            volumes: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::Volume>,
            > as crate::OptionableConvert>::into_optioned(self.volumes),
        }
    }
    fn try_from_optioned(value: PodSpecOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            active_deadline_seconds: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(
                value.active_deadline_seconds,
            )?,
            affinity: <Option<
                ::k8s_openapi::api::core::v1::Affinity,
            > as crate::OptionableConvert>::try_from_optioned(value.affinity)?,
            automount_service_account_token: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(
                value.automount_service_account_token,
            )?,
            containers: <std::vec::Vec<
                ::k8s_openapi::api::core::v1::Container,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .containers
                    .ok_or(crate::optionable::Error {
                        missing_field: "containers",
                    })?,
            )?,
            dns_config: <Option<
                ::k8s_openapi::api::core::v1::PodDNSConfig,
            > as crate::OptionableConvert>::try_from_optioned(value.dns_config)?,
            dns_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.dns_policy)?,
            enable_service_links: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(
                value.enable_service_links,
            )?,
            ephemeral_containers: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::EphemeralContainer>,
            > as crate::OptionableConvert>::try_from_optioned(
                value.ephemeral_containers,
            )?,
            host_aliases: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::HostAlias>,
            > as crate::OptionableConvert>::try_from_optioned(value.host_aliases)?,
            host_ipc: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.host_ipc)?,
            host_network: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.host_network)?,
            host_pid: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.host_pid)?,
            host_users: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.host_users)?,
            hostname: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.hostname)?,
            hostname_override: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.hostname_override)?,
            image_pull_secrets: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::LocalObjectReference>,
            > as crate::OptionableConvert>::try_from_optioned(value.image_pull_secrets)?,
            init_containers: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::Container>,
            > as crate::OptionableConvert>::try_from_optioned(value.init_containers)?,
            node_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.node_name)?,
            node_selector: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.node_selector)?,
            os: <Option<
                ::k8s_openapi::api::core::v1::PodOS,
            > as crate::OptionableConvert>::try_from_optioned(value.os)?,
            overhead: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.overhead)?,
            preemption_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.preemption_policy)?,
            priority: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.priority)?,
            priority_class_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.priority_class_name,
            )?,
            readiness_gates: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::PodReadinessGate>,
            > as crate::OptionableConvert>::try_from_optioned(value.readiness_gates)?,
            resource_claims: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::PodResourceClaim>,
            > as crate::OptionableConvert>::try_from_optioned(value.resource_claims)?,
            resources: <Option<
                ::k8s_openapi::api::core::v1::ResourceRequirements,
            > as crate::OptionableConvert>::try_from_optioned(value.resources)?,
            restart_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.restart_policy)?,
            runtime_class_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.runtime_class_name)?,
            scheduler_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.scheduler_name)?,
            scheduling_gates: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::PodSchedulingGate>,
            > as crate::OptionableConvert>::try_from_optioned(value.scheduling_gates)?,
            security_context: <Option<
                ::k8s_openapi::api::core::v1::PodSecurityContext,
            > as crate::OptionableConvert>::try_from_optioned(value.security_context)?,
            service_account: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.service_account)?,
            service_account_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.service_account_name,
            )?,
            set_hostname_as_fqdn: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(
                value.set_hostname_as_fqdn,
            )?,
            share_process_namespace: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(
                value.share_process_namespace,
            )?,
            subdomain: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.subdomain)?,
            termination_grace_period_seconds: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(
                value.termination_grace_period_seconds,
            )?,
            tolerations: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::Toleration>,
            > as crate::OptionableConvert>::try_from_optioned(value.tolerations)?,
            topology_spread_constraints: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::TopologySpreadConstraint>,
            > as crate::OptionableConvert>::try_from_optioned(
                value.topology_spread_constraints,
            )?,
            volumes: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::Volume>,
            > as crate::OptionableConvert>::try_from_optioned(value.volumes)?,
        })
    }
    fn merge(&mut self, other: PodSpecOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(
            &mut self.active_deadline_seconds,
            other.active_deadline_seconds,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::Affinity,
        > as crate::OptionableConvert>::merge(&mut self.affinity, other.affinity)?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.automount_service_account_token,
            other.automount_service_account_token,
        )?;
        if let Some(other_value) = other.containers {
            <std::vec::Vec<
                ::k8s_openapi::api::core::v1::Container,
            > as crate::OptionableConvert>::merge(&mut self.containers, other_value)?;
        }
        <Option<
            ::k8s_openapi::api::core::v1::PodDNSConfig,
        > as crate::OptionableConvert>::merge(&mut self.dns_config, other.dns_config)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.dns_policy, other.dns_policy)?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.enable_service_links,
            other.enable_service_links,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::EphemeralContainer>,
        > as crate::OptionableConvert>::merge(
            &mut self.ephemeral_containers,
            other.ephemeral_containers,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::HostAlias>,
        > as crate::OptionableConvert>::merge(
            &mut self.host_aliases,
            other.host_aliases,
        )?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.host_ipc, other.host_ipc)?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.host_network,
            other.host_network,
        )?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.host_pid, other.host_pid)?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.host_users, other.host_users)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.hostname, other.hostname)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.hostname_override,
            other.hostname_override,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::LocalObjectReference>,
        > as crate::OptionableConvert>::merge(
            &mut self.image_pull_secrets,
            other.image_pull_secrets,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::Container>,
        > as crate::OptionableConvert>::merge(
            &mut self.init_containers,
            other.init_containers,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.node_name, other.node_name)?;
        <Option<
            std::collections::BTreeMap<std::string::String, std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.node_selector,
            other.node_selector,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::PodOS,
        > as crate::OptionableConvert>::merge(&mut self.os, other.os)?;
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            >,
        > as crate::OptionableConvert>::merge(&mut self.overhead, other.overhead)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.preemption_policy,
            other.preemption_policy,
        )?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(&mut self.priority, other.priority)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.priority_class_name,
            other.priority_class_name,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::PodReadinessGate>,
        > as crate::OptionableConvert>::merge(
            &mut self.readiness_gates,
            other.readiness_gates,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::PodResourceClaim>,
        > as crate::OptionableConvert>::merge(
            &mut self.resource_claims,
            other.resource_claims,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::ResourceRequirements,
        > as crate::OptionableConvert>::merge(&mut self.resources, other.resources)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.restart_policy,
            other.restart_policy,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.runtime_class_name,
            other.runtime_class_name,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.scheduler_name,
            other.scheduler_name,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::PodSchedulingGate>,
        > as crate::OptionableConvert>::merge(
            &mut self.scheduling_gates,
            other.scheduling_gates,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::PodSecurityContext,
        > as crate::OptionableConvert>::merge(
            &mut self.security_context,
            other.security_context,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.service_account,
            other.service_account,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.service_account_name,
            other.service_account_name,
        )?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.set_hostname_as_fqdn,
            other.set_hostname_as_fqdn,
        )?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.share_process_namespace,
            other.share_process_namespace,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.subdomain, other.subdomain)?;
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(
            &mut self.termination_grace_period_seconds,
            other.termination_grace_period_seconds,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::Toleration>,
        > as crate::OptionableConvert>::merge(&mut self.tolerations, other.tolerations)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::TopologySpreadConstraint>,
        > as crate::OptionableConvert>::merge(
            &mut self.topology_spread_constraints,
            other.topology_spread_constraints,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::Volume>,
        > as crate::OptionableConvert>::merge(&mut self.volumes, other.volumes)?;
        Ok(())
    }
}
