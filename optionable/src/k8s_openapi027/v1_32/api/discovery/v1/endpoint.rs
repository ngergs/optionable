#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EndpointAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        <::k8s_openapi027::api::discovery::v1::EndpointConditions as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_topology: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints: Option<
        <::k8s_openapi027::api::discovery::v1::EndpointHints as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<
        <::k8s_openapi027::api::core::v1::ObjectReference as crate::Optionable>::Optioned,
    >,
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
