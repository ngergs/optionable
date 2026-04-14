#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ServiceSpec describes the attributes that a user creates on a service.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServiceSpecAc {
    /// allocateLoadBalancerNodePorts defines if NodePorts will be automatically allocated for services with type LoadBalancer.  Default is "true". It may be set to "false" if the cluster load-balancer does not rely on NodePorts.  If the caller requests specific NodePorts (by specifying a value), those requests will be respected, regardless of this field. This field may only be set for services with type LoadBalancer and will be cleared if the type is changed to any other type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocate_load_balancer_node_ports: Option<bool>,
    /// clusterIP is the IP address of the service and is usually assigned randomly. If an address is specified manually, is in-range (as per system configuration), and is not in use, it will be allocated to the service; otherwise creation of the service will fail. This field may not be changed through updates unless the type field is also being changed to ExternalName (which requires this field to be blank) or the type field is being changed from ExternalName (in which case this field may optionally be specified, as describe above).  Valid values are "None", empty string (""), or a valid IP address. Setting this to "None" makes a "headless service" (no virtual IP), which is useful when direct endpoint connections are preferred and proxying is not required.  Only applies to types ClusterIP, NodePort, and LoadBalancer. If this field is specified when creating a Service of type ExternalName, creation will fail. This field will be wiped when updating a Service to type ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
    #[serde(rename = "clusterIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_ip: Option<std::string::String>,
    /// ClusterIPs is a list of IP addresses assigned to this service, and are usually assigned randomly.  If an address is specified manually, is in-range (as per system configuration), and is not in use, it will be allocated to the service; otherwise creation of the service will fail. This field may not be changed through updates unless the type field is also being changed to ExternalName (which requires this field to be empty) or the type field is being changed from ExternalName (in which case this field may optionally be specified, as describe above).  Valid values are "None", empty string (""), or a valid IP address.  Setting this to "None" makes a "headless service" (no virtual IP), which is useful when direct endpoint connections are preferred and proxying is not required.  Only applies to types ClusterIP, NodePort, and LoadBalancer. If this field is specified when creating a Service of type ExternalName, creation will fail. This field will be wiped when updating a Service to type ExternalName.  If this field is not specified, it will be initialized from the clusterIP field.  If this field is specified, clients must ensure that clusterIPs\[0\] and clusterIP have the same value.
    ///
    /// This field may hold a maximum of two entries (dual-stack IPs, in either order). These IPs must correspond to the values of the ipFamilies field. Both clusterIPs and ipFamilies are governed by the ipFamilyPolicy field. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
    #[serde(rename = "clusterIPs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_ips: Option<std::vec::Vec<std::string::String>>,
    /// externalIPs is a list of IP addresses for which nodes in the cluster will also accept traffic for this service.  These IPs are not managed by Kubernetes.  The user is responsible for ensuring that traffic arrives at a node with this IP.  A common example is external load-balancers that are not part of the Kubernetes system.
    #[serde(rename = "externalIPs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ips: Option<std::vec::Vec<std::string::String>>,
    /// externalName is the external reference that discovery mechanisms will return as an alias for this service (e.g. a DNS CNAME record). No proxying will be involved.  Must be a lowercase RFC-1123 hostname (https://tools.ietf.org/html/rfc1123) and requires `type` to be "ExternalName".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_name: Option<std::string::String>,
    /// externalTrafficPolicy describes how nodes distribute service traffic they receive on one of the Service's "externally-facing" addresses (NodePorts, ExternalIPs, and LoadBalancer IPs). If set to "Local", the proxy will configure the service in a way that assumes that external load balancers will take care of balancing the service traffic between nodes, and so each node will deliver traffic only to the node-local endpoints of the service, without masquerading the client source IP. (Traffic mistakenly sent to a node with no endpoints will be dropped.) The default value, "Cluster", uses the standard behavior of routing to all endpoints evenly (possibly modified by topology and other features). Note that traffic sent to an External IP or LoadBalancer IP from within the cluster will always get "Cluster" semantics, but clients sending to a NodePort from within the cluster may need to take traffic policy into account when picking a node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_traffic_policy: Option<std::string::String>,
    /// healthCheckNodePort specifies the healthcheck nodePort for the service. This only applies when type is set to LoadBalancer and externalTrafficPolicy is set to Local. If a value is specified, is in-range, and is not in use, it will be used.  If not specified, a value will be automatically allocated.  External systems (e.g. load-balancers) can use this port to determine if a given node holds endpoints for this service or not.  If this field is specified when creating a Service which does not need it, creation will fail. This field will be wiped when updating a Service to no longer need it (e.g. changing type). This field cannot be updated once set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_node_port: Option<i32>,
    /// InternalTrafficPolicy describes how nodes distribute service traffic they receive on the ClusterIP. If set to "Local", the proxy will assume that pods only want to talk to endpoints of the service on the same node as the pod, dropping the traffic if there are no local endpoints. The default value, "Cluster", uses the standard behavior of routing to all endpoints evenly (possibly modified by topology and other features).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_traffic_policy: Option<std::string::String>,
    /// IPFamilies is a list of IP families (e.g. IPv4, IPv6) assigned to this service. This field is usually assigned automatically based on cluster configuration and the ipFamilyPolicy field. If this field is specified manually, the requested family is available in the cluster, and ipFamilyPolicy allows it, it will be used; otherwise creation of the service will fail. This field is conditionally mutable: it allows for adding or removing a secondary IP family, but it does not allow changing the primary IP family of the Service. Valid values are "IPv4" and "IPv6".  This field only applies to Services of types ClusterIP, NodePort, and LoadBalancer, and does apply to "headless" services. This field will be wiped when updating a Service to type ExternalName.
    ///
    /// This field may hold a maximum of two entries (dual-stack families, in either order).  These families must correspond to the values of the clusterIPs field, if specified. Both clusterIPs and ipFamilies are governed by the ipFamilyPolicy field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_families: Option<std::vec::Vec<std::string::String>>,
    /// IPFamilyPolicy represents the dual-stack-ness requested or required by this Service. If there is no value provided, then this field will be set to SingleStack. Services can be "SingleStack" (a single IP family), "PreferDualStack" (two IP families on dual-stack configured clusters or a single IP family on single-stack clusters), or "RequireDualStack" (two IP families on dual-stack configured clusters, otherwise fail). The ipFamilies and clusterIPs fields depend on the value of this field. This field will be wiped when updating a service to type ExternalName.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_family_policy: Option<std::string::String>,
    /// loadBalancerClass is the class of the load balancer implementation this Service belongs to. If specified, the value of this field must be a label-style identifier, with an optional prefix, e.g. "internal-vip" or "example.com/internal-vip". Unprefixed names are reserved for end-users. This field can only be set when the Service type is 'LoadBalancer'. If not set, the default load balancer implementation is used, today this is typically done through the cloud provider integration, but should apply for any default implementation. If set, it is assumed that a load balancer implementation is watching for Services with a matching class. Any default load balancer implementation (e.g. cloud providers) should ignore Services that set this field. This field can only be set when creating or updating a Service to type 'LoadBalancer'. Once set, it can not be changed. This field will be wiped when a service is updated to a non 'LoadBalancer' type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_class: Option<std::string::String>,
    /// Only applies to Service Type: LoadBalancer. This feature depends on whether the underlying cloud-provider supports specifying the loadBalancerIP when a load balancer is created. This field will be ignored if the cloud-provider does not support the feature. Deprecated: This field was under-specified and its meaning varies across implementations. Using it is non-portable and it may not support dual-stack. Users are encouraged to use implementation-specific annotations when available.
    #[serde(rename = "loadBalancerIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_ip: Option<std::string::String>,
    /// If specified and supported by the platform, this will restrict traffic through the cloud-provider load-balancer will be restricted to the specified client IPs. This field will be ignored if the cloud-provider does not support the feature." More info: https://kubernetes.io/docs/tasks/access-application-cluster/create-external-load-balancer/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_source_ranges: Option<std::vec::Vec<std::string::String>>,
    /// The list of ports that are exposed by this service. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::ServicePort as crate::Optionable>::Optioned,
        >,
    >,
    /// publishNotReadyAddresses indicates that any agent which deals with endpoints for this Service should disregard any indications of ready/not-ready. The primary use case for setting this field is for a StatefulSet's Headless Service to propagate SRV DNS records for its Pods for the purpose of peer discovery. The Kubernetes controllers that generate Endpoints and EndpointSlice resources for Services interpret this to mean that all endpoints are considered "ready" even if the Pods themselves are not. Agents which consume only Kubernetes generated endpoints through the Endpoints or EndpointSlice resources can safely assume this behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_not_ready_addresses: Option<bool>,
    /// Route service traffic to pods with label keys and values matching this selector. If empty or not present, the service is assumed to have an external process managing its endpoints, which Kubernetes will not modify. Only applies to types ClusterIP, NodePort, and LoadBalancer. Ignored if type is ExternalName. More info: https://kubernetes.io/docs/concepts/services-networking/service/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
    /// Supports "ClientIP" and "None". Used to maintain session affinity. Enable client IP based session affinity. Must be ClientIP or None. Defaults to None. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_affinity: Option<std::string::String>,
    /// sessionAffinityConfig contains the configurations of session affinity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_affinity_config: Option<
        <::k8s_openapi027::api::core::v1::SessionAffinityConfig as crate::Optionable>::Optioned,
    >,
    /// TrafficDistribution offers a way to express preferences for how traffic is distributed to Service endpoints. Implementations can use this field as a hint, but are not required to guarantee strict adherence. If the field is not set, the implementation will apply its default routing strategy. If set to "PreferClose", implementations should prioritize endpoints that are topologically close (e.g., same zone). This is a beta field and requires enabling ServiceTrafficDistribution feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_distribution: Option<std::string::String>,
    /// type determines how the Service is exposed. Defaults to ClusterIP. Valid options are ExternalName, ClusterIP, NodePort, and LoadBalancer. "ClusterIP" allocates a cluster-internal IP address for load-balancing to endpoints. Endpoints are determined by the selector or if that is not specified, by manual construction of an Endpoints object or EndpointSlice objects. If clusterIP is "None", no virtual IP is allocated and the endpoints are published as a set of endpoints rather than a virtual IP. "NodePort" builds on ClusterIP and allocates a port on every node which routes to the same endpoints as the clusterIP. "LoadBalancer" builds on NodePort and creates an external load-balancer (if supported in the current cloud) which routes to the same endpoints as the clusterIP. "ExternalName" aliases this service to the specified externalName. Several other fields do not apply to ExternalName services. More info: https://kubernetes.io/docs/concepts/services-networking/service/#publishing-services-service-types
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
        if self.allocate_load_balancer_node_ports.is_none() {
            self.allocate_load_balancer_node_ports = crate::OptionableConvert::try_from_optioned(
                other.allocate_load_balancer_node_ports,
            )?;
        } else if let Some(self_value) = self.allocate_load_balancer_node_ports.as_mut()
            && let Some(other_value) = other.allocate_load_balancer_node_ports
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.cluster_ip.is_none() {
            self.cluster_ip = crate::OptionableConvert::try_from_optioned(
                other.cluster_ip,
            )?;
        } else if let Some(self_value) = self.cluster_ip.as_mut()
            && let Some(other_value) = other.cluster_ip
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.cluster_ips.is_none() {
            self.cluster_ips = crate::OptionableConvert::try_from_optioned(
                other.cluster_ips,
            )?;
        } else if let Some(self_value) = self.cluster_ips.as_mut()
            && let Some(other_value) = other.cluster_ips
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.external_ips.is_none() {
            self.external_ips = crate::OptionableConvert::try_from_optioned(
                other.external_ips,
            )?;
        } else if let Some(self_value) = self.external_ips.as_mut()
            && let Some(other_value) = other.external_ips
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.external_name.is_none() {
            self.external_name = crate::OptionableConvert::try_from_optioned(
                other.external_name,
            )?;
        } else if let Some(self_value) = self.external_name.as_mut()
            && let Some(other_value) = other.external_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.external_traffic_policy.is_none() {
            self.external_traffic_policy = crate::OptionableConvert::try_from_optioned(
                other.external_traffic_policy,
            )?;
        } else if let Some(self_value) = self.external_traffic_policy.as_mut()
            && let Some(other_value) = other.external_traffic_policy
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.health_check_node_port.is_none() {
            self.health_check_node_port = crate::OptionableConvert::try_from_optioned(
                other.health_check_node_port,
            )?;
        } else if let Some(self_value) = self.health_check_node_port.as_mut()
            && let Some(other_value) = other.health_check_node_port
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.internal_traffic_policy.is_none() {
            self.internal_traffic_policy = crate::OptionableConvert::try_from_optioned(
                other.internal_traffic_policy,
            )?;
        } else if let Some(self_value) = self.internal_traffic_policy.as_mut()
            && let Some(other_value) = other.internal_traffic_policy
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.ip_families.is_none() {
            self.ip_families = crate::OptionableConvert::try_from_optioned(
                other.ip_families,
            )?;
        } else if let Some(self_value) = self.ip_families.as_mut()
            && let Some(other_value) = other.ip_families
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.ip_family_policy.is_none() {
            self.ip_family_policy = crate::OptionableConvert::try_from_optioned(
                other.ip_family_policy,
            )?;
        } else if let Some(self_value) = self.ip_family_policy.as_mut()
            && let Some(other_value) = other.ip_family_policy
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.load_balancer_class.is_none() {
            self.load_balancer_class = crate::OptionableConvert::try_from_optioned(
                other.load_balancer_class,
            )?;
        } else if let Some(self_value) = self.load_balancer_class.as_mut()
            && let Some(other_value) = other.load_balancer_class
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.load_balancer_ip.is_none() {
            self.load_balancer_ip = crate::OptionableConvert::try_from_optioned(
                other.load_balancer_ip,
            )?;
        } else if let Some(self_value) = self.load_balancer_ip.as_mut()
            && let Some(other_value) = other.load_balancer_ip
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.load_balancer_source_ranges.is_none() {
            self.load_balancer_source_ranges = crate::OptionableConvert::try_from_optioned(
                other.load_balancer_source_ranges,
            )?;
        } else if let Some(self_value) = self.load_balancer_source_ranges.as_mut()
            && let Some(other_value) = other.load_balancer_source_ranges
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.ports.is_none() {
            self.ports = crate::OptionableConvert::try_from_optioned(other.ports)?;
        } else if let Some(self_value) = self.ports.as_mut()
            && let Some(other_value) = other.ports
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if self.publish_not_ready_addresses.is_none() {
            self.publish_not_ready_addresses = crate::OptionableConvert::try_from_optioned(
                other.publish_not_ready_addresses,
            )?;
        } else if let Some(self_value) = self.publish_not_ready_addresses.as_mut()
            && let Some(other_value) = other.publish_not_ready_addresses
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.selector.is_none() {
            self.selector = crate::OptionableConvert::try_from_optioned(other.selector)?;
        } else if let Some(self_value) = self.selector.as_mut()
            && let Some(other_value) = other.selector
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.session_affinity.is_none() {
            self.session_affinity = crate::OptionableConvert::try_from_optioned(
                other.session_affinity,
            )?;
        } else if let Some(self_value) = self.session_affinity.as_mut()
            && let Some(other_value) = other.session_affinity
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.session_affinity_config.is_none() {
            self.session_affinity_config = crate::OptionableConvert::try_from_optioned(
                other.session_affinity_config,
            )?;
        } else if let Some(self_value) = self.session_affinity_config.as_mut()
            && let Some(other_value) = other.session_affinity_config
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.traffic_distribution.is_none() {
            self.traffic_distribution = crate::OptionableConvert::try_from_optioned(
                other.traffic_distribution,
            )?;
        } else if let Some(self_value) = self.traffic_distribution.as_mut()
            && let Some(other_value) = other.traffic_distribution
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.type_.is_none() {
            self.type_ = crate::OptionableConvert::try_from_optioned(other.type_)?;
        } else if let Some(self_value) = self.type_.as_mut()
            && let Some(other_value) = other.type_
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
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
