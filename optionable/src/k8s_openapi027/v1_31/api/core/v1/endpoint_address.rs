#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EndpointAddressAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<std::string::String>,
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
        self.hostname = other.hostname;
        if let Some(other_value) = other.ip {
            self.ip = other_value;
        }
        self.node_name = other.node_name;
        crate::OptionableConvert::merge(&mut self.target_ref, other.target_ref)?;
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
