#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NodeAddress contains information for the node's address.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeAddressAc {
    /// The node address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<std::string::String>,
    /// Node address type, one of Hostname, ExternalIP or InternalIP.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::NodeAddress {
    type Optioned = NodeAddressAc;
}
#[automatically_derived]
impl crate::Optionable for NodeAddressAc {
    type Optioned = NodeAddressAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::NodeAddress {
    fn into_optioned(self) -> NodeAddressAc {
        NodeAddressAc {
            address: Some(self.address),
            type_: Some(self.type_),
        }
    }
    fn try_from_optioned(value: NodeAddressAc) -> Result<Self, crate::Error> {
        Ok(Self {
            address: value
                .address
                .ok_or(crate::Error {
                    missing_field: "address",
                })?,
            type_: value
                .type_
                .ok_or(crate::Error {
                    missing_field: "type_",
                })?,
        })
    }
    fn merge(&mut self, other: NodeAddressAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.address {
            self.address = other_value;
        }
        if let Some(other_value) = other.type_ {
            self.type_ = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::NodeAddress>
for NodeAddressAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::NodeAddress) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::NodeAddress, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::NodeAddress,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
