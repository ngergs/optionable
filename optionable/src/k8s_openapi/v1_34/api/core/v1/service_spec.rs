pub struct ServiceSpecOpt {
    pub allocate_load_balancer_node_ports: <Option<bool> as crate::Optionable>::Optioned,
    pub cluster_ip: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub cluster_ips: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub external_ips: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub external_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub external_traffic_policy: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub health_check_node_port: <Option<i32> as crate::Optionable>::Optioned,
    pub internal_traffic_policy: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub ip_families: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub ip_family_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub load_balancer_class: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub load_balancer_ip: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub load_balancer_source_ranges: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub ports: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ServicePort>,
    > as crate::Optionable>::Optioned,
    pub publish_not_ready_addresses: <Option<bool> as crate::Optionable>::Optioned,
    pub selector: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    pub session_affinity: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub session_affinity_config: <Option<
        ::k8s_openapi::api::core::v1::SessionAffinityConfig,
    > as crate::Optionable>::Optioned,
    pub traffic_distribution: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub type_: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ServiceSpec {
    type Optioned = ServiceSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for ServiceSpecOpt {
    type Optioned = ServiceSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ServiceSpec {
    fn into_optioned(self) -> ServiceSpecOpt {
        ServiceSpecOpt {
            allocate_load_balancer_node_ports: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(
                self.allocate_load_balancer_node_ports,
            ),
            cluster_ip: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.cluster_ip),
            cluster_ips: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.cluster_ips),
            external_ips: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.external_ips),
            external_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.external_name),
            external_traffic_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.external_traffic_policy),
            health_check_node_port: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.health_check_node_port),
            internal_traffic_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.internal_traffic_policy),
            ip_families: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.ip_families),
            ip_family_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.ip_family_policy),
            load_balancer_class: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.load_balancer_class),
            load_balancer_ip: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.load_balancer_ip),
            load_balancer_source_ranges: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(
                self.load_balancer_source_ranges,
            ),
            ports: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::ServicePort>,
            > as crate::OptionableConvert>::into_optioned(self.ports),
            publish_not_ready_addresses: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(
                self.publish_not_ready_addresses,
            ),
            selector: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.selector),
            session_affinity: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.session_affinity),
            session_affinity_config: <Option<
                ::k8s_openapi::api::core::v1::SessionAffinityConfig,
            > as crate::OptionableConvert>::into_optioned(self.session_affinity_config),
            traffic_distribution: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.traffic_distribution),
            type_: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.type_),
        }
    }
    fn try_from_optioned(
        value: ServiceSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            allocate_load_balancer_node_ports: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(
                value.allocate_load_balancer_node_ports,
            )?,
            cluster_ip: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.cluster_ip)?,
            cluster_ips: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.cluster_ips)?,
            external_ips: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.external_ips)?,
            external_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.external_name)?,
            external_traffic_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.external_traffic_policy,
            )?,
            health_check_node_port: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(
                value.health_check_node_port,
            )?,
            internal_traffic_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.internal_traffic_policy,
            )?,
            ip_families: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.ip_families)?,
            ip_family_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.ip_family_policy)?,
            load_balancer_class: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.load_balancer_class,
            )?,
            load_balancer_ip: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.load_balancer_ip)?,
            load_balancer_source_ranges: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(
                value.load_balancer_source_ranges,
            )?,
            ports: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::ServicePort>,
            > as crate::OptionableConvert>::try_from_optioned(value.ports)?,
            publish_not_ready_addresses: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(
                value.publish_not_ready_addresses,
            )?,
            selector: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.selector)?,
            session_affinity: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.session_affinity)?,
            session_affinity_config: <Option<
                ::k8s_openapi::api::core::v1::SessionAffinityConfig,
            > as crate::OptionableConvert>::try_from_optioned(
                value.session_affinity_config,
            )?,
            traffic_distribution: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.traffic_distribution,
            )?,
            type_: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.type_)?,
        })
    }
    fn merge(&mut self, other: ServiceSpecOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.allocate_load_balancer_node_ports,
            other.allocate_load_balancer_node_ports,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.cluster_ip, other.cluster_ip)?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.cluster_ips, other.cluster_ips)?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.external_ips,
            other.external_ips,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.external_name,
            other.external_name,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.external_traffic_policy,
            other.external_traffic_policy,
        )?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.health_check_node_port,
            other.health_check_node_port,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.internal_traffic_policy,
            other.internal_traffic_policy,
        )?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.ip_families, other.ip_families)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.ip_family_policy,
            other.ip_family_policy,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.load_balancer_class,
            other.load_balancer_class,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.load_balancer_ip,
            other.load_balancer_ip,
        )?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.load_balancer_source_ranges,
            other.load_balancer_source_ranges,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::ServicePort>,
        > as crate::OptionableConvert>::merge(&mut self.ports, other.ports)?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.publish_not_ready_addresses,
            other.publish_not_ready_addresses,
        )?;
        <Option<
            std::collections::BTreeMap<std::string::String, std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.selector, other.selector)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.session_affinity,
            other.session_affinity,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::SessionAffinityConfig,
        > as crate::OptionableConvert>::merge(
            &mut self.session_affinity_config,
            other.session_affinity_config,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.traffic_distribution,
            other.traffic_distribution,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.type_, other.type_)?;
        Ok(())
    }
}
