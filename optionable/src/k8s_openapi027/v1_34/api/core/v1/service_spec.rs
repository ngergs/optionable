#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServiceSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocate_load_balancer_node_ports: Option<bool>,
    #[serde(rename = "clusterIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_ip: Option<std::string::String>,
    #[serde(rename = "clusterIPs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_ips: Option<std::vec::Vec<std::string::String>>,
    #[serde(rename = "externalIPs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ips: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_traffic_policy: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_node_port: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_traffic_policy: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_families: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_family_policy: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_class: Option<std::string::String>,
    #[serde(rename = "loadBalancerIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_ip: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_source_ranges: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::ServicePort as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_not_ready_addresses: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_affinity: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_affinity_config: Option<
        <::k8s_openapi027::api::core::v1::SessionAffinityConfig as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_distribution: Option<std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ServiceSpec {
    type Optioned = ServiceSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ServiceSpecAc {
    type Optioned = ServiceSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ServiceSpec {
    fn into_optioned(self) -> ServiceSpecAc {
        ServiceSpecAc {
            allocate_load_balancer_node_ports: self.allocate_load_balancer_node_ports,
            cluster_ip: self.cluster_ip,
            cluster_ips: self.cluster_ips,
            external_ips: self.external_ips,
            external_name: self.external_name,
            external_traffic_policy: self.external_traffic_policy,
            health_check_node_port: self.health_check_node_port,
            internal_traffic_policy: self.internal_traffic_policy,
            ip_families: self.ip_families,
            ip_family_policy: self.ip_family_policy,
            load_balancer_class: self.load_balancer_class,
            load_balancer_ip: self.load_balancer_ip,
            load_balancer_source_ranges: self.load_balancer_source_ranges,
            ports: crate::OptionableConvert::into_optioned(self.ports),
            publish_not_ready_addresses: self.publish_not_ready_addresses,
            selector: self.selector,
            session_affinity: self.session_affinity,
            session_affinity_config: crate::OptionableConvert::into_optioned(
                self.session_affinity_config,
            ),
            traffic_distribution: self.traffic_distribution,
            type_: self.type_,
        }
    }
    fn try_from_optioned(value: ServiceSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            allocate_load_balancer_node_ports: value.allocate_load_balancer_node_ports,
            cluster_ip: value.cluster_ip,
            cluster_ips: value.cluster_ips,
            external_ips: value.external_ips,
            external_name: value.external_name,
            external_traffic_policy: value.external_traffic_policy,
            health_check_node_port: value.health_check_node_port,
            internal_traffic_policy: value.internal_traffic_policy,
            ip_families: value.ip_families,
            ip_family_policy: value.ip_family_policy,
            load_balancer_class: value.load_balancer_class,
            load_balancer_ip: value.load_balancer_ip,
            load_balancer_source_ranges: value.load_balancer_source_ranges,
            ports: crate::OptionableConvert::try_from_optioned(value.ports)?,
            publish_not_ready_addresses: value.publish_not_ready_addresses,
            selector: value.selector,
            session_affinity: value.session_affinity,
            session_affinity_config: crate::OptionableConvert::try_from_optioned(
                value.session_affinity_config,
            )?,
            traffic_distribution: value.traffic_distribution,
            type_: value.type_,
        })
    }
    fn merge(&mut self, other: ServiceSpecAc) -> Result<(), crate::Error> {
        self.allocate_load_balancer_node_ports = other.allocate_load_balancer_node_ports;
        self.cluster_ip = other.cluster_ip;
        self.cluster_ips = other.cluster_ips;
        self.external_ips = other.external_ips;
        self.external_name = other.external_name;
        self.external_traffic_policy = other.external_traffic_policy;
        self.health_check_node_port = other.health_check_node_port;
        self.internal_traffic_policy = other.internal_traffic_policy;
        self.ip_families = other.ip_families;
        self.ip_family_policy = other.ip_family_policy;
        self.load_balancer_class = other.load_balancer_class;
        self.load_balancer_ip = other.load_balancer_ip;
        self.load_balancer_source_ranges = other.load_balancer_source_ranges;
        crate::OptionableConvert::merge(&mut self.ports, other.ports)?;
        self.publish_not_ready_addresses = other.publish_not_ready_addresses;
        self.selector = other.selector;
        self.session_affinity = other.session_affinity;
        crate::OptionableConvert::merge(
            &mut self.session_affinity_config,
            other.session_affinity_config,
        )?;
        self.traffic_distribution = other.traffic_distribution;
        self.type_ = other.type_;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ServiceSpec>
for ServiceSpecAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::ServiceSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ServiceSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ServiceSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
