#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct EndpointAddressAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ref: <Option<
        ::k8s_openapi::api::core::v1::ObjectReference,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::EndpointAddress {
    type Optioned = EndpointAddressAc;
}
#[automatically_derived]
impl crate::Optionable for EndpointAddressAc {
    type Optioned = EndpointAddressAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::EndpointAddress {
    fn into_optioned(self) -> EndpointAddressAc {
        EndpointAddressAc {
            hostname: crate::OptionableConvert::into_optioned(self.hostname),
            ip: Some(crate::OptionableConvert::into_optioned(self.ip)),
            node_name: crate::OptionableConvert::into_optioned(self.node_name),
            target_ref: crate::OptionableConvert::into_optioned(self.target_ref),
        }
    }
    fn try_from_optioned(value: EndpointAddressAc) -> Result<Self, crate::Error> {
        Ok(Self {
            hostname: crate::OptionableConvert::try_from_optioned(value.hostname)?,
            ip: crate::OptionableConvert::try_from_optioned(
                value
                    .ip
                    .ok_or(crate::Error {
                        missing_field: "ip",
                    })?,
            )?,
            node_name: crate::OptionableConvert::try_from_optioned(value.node_name)?,
            target_ref: crate::OptionableConvert::try_from_optioned(value.target_ref)?,
        })
    }
    fn merge(&mut self, other: EndpointAddressAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.hostname, other.hostname)?;
        if let Some(other_value) = other.ip {
            crate::OptionableConvert::merge(&mut self.ip, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.node_name, other.node_name)?;
        crate::OptionableConvert::merge(&mut self.target_ref, other.target_ref)?;
        Ok(())
    }
}
