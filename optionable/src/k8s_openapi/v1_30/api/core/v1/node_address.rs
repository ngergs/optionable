#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct NodeAddressAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NodeAddress {
    type Optioned = NodeAddressAc;
}
#[automatically_derived]
impl crate::Optionable for NodeAddressAc {
    type Optioned = NodeAddressAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NodeAddress {
    fn into_optioned(self) -> NodeAddressAc {
        NodeAddressAc {
            address: Some(crate::OptionableConvert::into_optioned(self.address)),
            type_: Some(crate::OptionableConvert::into_optioned(self.type_)),
        }
    }
    fn try_from_optioned(value: NodeAddressAc) -> Result<Self, crate::Error> {
        Ok(Self {
            address: crate::OptionableConvert::try_from_optioned(
                value
                    .address
                    .ok_or(crate::Error {
                        missing_field: "address",
                    })?,
            )?,
            type_: crate::OptionableConvert::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::Error {
                        missing_field: "type_",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: NodeAddressAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.address {
            crate::OptionableConvert::merge(&mut self.address, other_value)?;
        }
        if let Some(other_value) = other.type_ {
            crate::OptionableConvert::merge(&mut self.type_, other_value)?;
        }
        Ok(())
    }
}
