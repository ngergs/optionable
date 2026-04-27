#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// EndpointAddress is a tuple that describes single IP address.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EndpointAddressAc {
    /// The Hostname of this endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<std::string::String>,
    /// The IP of this endpoint. May not be loopback (127.0.0.0/8 or ::1), link-local (169.254.0.0/16 or fe80::/10), or link-local multicast (224.0.0.0/24 or ff02::/16).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<std::string::String>,
    /// Optional: Node hosting this endpoint. This can be used to determine endpoints local to a node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<std::string::String>,
    /// Reference to object providing the endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<
        <::k8s_openapi027::api::core::v1::ObjectReference as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::EndpointAddress {
    type Optioned = EndpointAddressAc;
}
#[automatically_derived]
impl crate::Optionable for EndpointAddressAc {
    type Optioned = EndpointAddressAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::EndpointAddress {
    fn into_optioned(self) -> EndpointAddressAc {
        EndpointAddressAc {
            hostname: self.hostname,
            ip: Some(self.ip),
            node_name: self.node_name,
            target_ref: crate::OptionableConvert::into_optioned(self.target_ref),
        }
    }
    fn try_from_optioned(value: EndpointAddressAc) -> Result<Self, crate::Error> {
        Ok(Self {
            hostname: value.hostname,
            ip: value
                .ip
                .ok_or(crate::Error {
                    missing_field: "ip",
                })?,
            node_name: value.node_name,
            target_ref: crate::OptionableConvert::try_from_optioned(value.target_ref)?,
        })
    }
    fn merge(&mut self, other: EndpointAddressAc) -> Result<(), crate::Error> {
        if self.hostname.is_none() {
            self.hostname = crate::OptionableConvert::try_from_optioned(other.hostname)?;
        } else if let Some(self_value) = self.hostname.as_mut()
            && let Some(other_value) = other.hostname
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.ip {
            self.ip = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.node_name.is_none() {
            self.node_name = crate::OptionableConvert::try_from_optioned(
                other.node_name,
            )?;
        } else if let Some(self_value) = self.node_name.as_mut()
            && let Some(other_value) = other.node_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.target_ref.is_none() {
            self.target_ref = crate::OptionableConvert::try_from_optioned(
                other.target_ref,
            )?;
        } else if let Some(self_value) = self.target_ref.as_mut()
            && let Some(other_value) = other.target_ref
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::EndpointAddress>
for EndpointAddressAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::EndpointAddress) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::EndpointAddress, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::EndpointAddress,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for EndpointAddressAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.hostname, other.hostname);
        k8s_openapi027::DeepMerge::merge_from(&mut self.ip, other.ip);
        k8s_openapi027::DeepMerge::merge_from(&mut self.node_name, other.node_name);
        self.target_ref = other.target_ref;
    }
}
