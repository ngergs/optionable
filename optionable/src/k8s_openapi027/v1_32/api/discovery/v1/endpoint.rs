#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Endpoint represents a single logical "backend" implementing a service.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EndpointAc {
    /// addresses of this endpoint. The contents of this field are interpreted according to the corresponding EndpointSlice addressType field. Consumers must handle different types of addresses in the context of their own capabilities. This must contain at least one address but no more than 100. These are all assumed to be fungible and clients may choose to only use the first element. Refer to: https://issue.k8s.io/106267
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<std::vec::Vec<std::string::String>>,
    /// conditions contains information about the current status of the endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        <::k8s_openapi027::api::discovery::v1::EndpointConditions as crate::Optionable>::Optioned,
    >,
    /// deprecatedTopology contains topology information part of the v1beta1 API. This field is deprecated, and will be removed when the v1beta1 API is removed (no sooner than kubernetes v1.24).  While this field can hold values, it is not writable through the v1 API, and any attempts to write to it will be silently ignored. Topology information can be found in the zone and nodeName fields instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_topology: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
    /// hints contains information associated with how an endpoint should be consumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints: Option<
        <::k8s_openapi027::api::discovery::v1::EndpointHints as crate::Optionable>::Optioned,
    >,
    /// hostname of this endpoint. This field may be used by consumers of endpoints to distinguish endpoints from each other (e.g. in DNS names). Multiple endpoints which use the same hostname should be considered fungible (e.g. multiple A values in DNS). Must be lowercase and pass DNS Label (RFC 1123) validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<std::string::String>,
    /// nodeName represents the name of the Node hosting this endpoint. This can be used to determine endpoints local to a Node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<std::string::String>,
    /// targetRef is a reference to a Kubernetes object that represents this endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<
        <::k8s_openapi027::api::core::v1::ObjectReference as crate::Optionable>::Optioned,
    >,
    /// zone is the name of the Zone this endpoint exists in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::discovery::v1::Endpoint {
    type Optioned = EndpointAc;
}
#[automatically_derived]
impl crate::Optionable for EndpointAc {
    type Optioned = EndpointAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::discovery::v1::Endpoint {
    fn into_optioned(self) -> EndpointAc {
        EndpointAc {
            addresses: Some(self.addresses),
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            deprecated_topology: self.deprecated_topology,
            hints: crate::OptionableConvert::into_optioned(self.hints),
            hostname: self.hostname,
            node_name: self.node_name,
            target_ref: crate::OptionableConvert::into_optioned(self.target_ref),
            zone: self.zone,
        }
    }
    fn try_from_optioned(value: EndpointAc) -> Result<Self, crate::Error> {
        Ok(Self {
            addresses: value
                .addresses
                .ok_or(crate::Error {
                    missing_field: "addresses",
                })?,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            deprecated_topology: value.deprecated_topology,
            hints: crate::OptionableConvert::try_from_optioned(value.hints)?,
            hostname: value.hostname,
            node_name: value.node_name,
            target_ref: crate::OptionableConvert::try_from_optioned(value.target_ref)?,
            zone: value.zone,
        })
    }
    fn merge(&mut self, other: EndpointAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.addresses {
            self.addresses = other_value;
        }
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        self.deprecated_topology = other.deprecated_topology;
        crate::OptionableConvert::merge(&mut self.hints, other.hints)?;
        self.hostname = other.hostname;
        self.node_name = other.node_name;
        crate::OptionableConvert::merge(&mut self.target_ref, other.target_ref)?;
        self.zone = other.zone;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::discovery::v1::Endpoint>
for EndpointAc {
    fn from_optionable(value: k8s_openapi027::api::discovery::v1::Endpoint) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::discovery::v1::Endpoint, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::discovery::v1::Endpoint,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
