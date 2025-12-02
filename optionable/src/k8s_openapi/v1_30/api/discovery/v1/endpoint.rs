#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct EndpointAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        ::k8s_openapi::api::discovery::v1::EndpointConditions,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_topology: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints: <Option<
        ::k8s_openapi::api::discovery::v1::EndpointHints,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ref: <Option<
        ::k8s_openapi::api::core::v1::ObjectReference,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::discovery::v1::Endpoint {
    type Optioned = EndpointAc;
}
#[automatically_derived]
impl crate::Optionable for EndpointAc {
    type Optioned = EndpointAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::discovery::v1::Endpoint {
    fn into_optioned(self) -> EndpointAc {
        EndpointAc {
            addresses: Some(crate::OptionableConvert::into_optioned(self.addresses)),
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            deprecated_topology: crate::OptionableConvert::into_optioned(
                self.deprecated_topology,
            ),
            hints: crate::OptionableConvert::into_optioned(self.hints),
            hostname: crate::OptionableConvert::into_optioned(self.hostname),
            node_name: crate::OptionableConvert::into_optioned(self.node_name),
            target_ref: crate::OptionableConvert::into_optioned(self.target_ref),
            zone: crate::OptionableConvert::into_optioned(self.zone),
        }
    }
    fn try_from_optioned(value: EndpointAc) -> Result<Self, crate::Error> {
        Ok(Self {
            addresses: crate::OptionableConvert::try_from_optioned(
                value
                    .addresses
                    .ok_or(crate::Error {
                        missing_field: "addresses",
                    })?,
            )?,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            deprecated_topology: crate::OptionableConvert::try_from_optioned(
                value.deprecated_topology,
            )?,
            hints: crate::OptionableConvert::try_from_optioned(value.hints)?,
            hostname: crate::OptionableConvert::try_from_optioned(value.hostname)?,
            node_name: crate::OptionableConvert::try_from_optioned(value.node_name)?,
            target_ref: crate::OptionableConvert::try_from_optioned(value.target_ref)?,
            zone: crate::OptionableConvert::try_from_optioned(value.zone)?,
        })
    }
    fn merge(&mut self, other: EndpointAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.addresses {
            crate::OptionableConvert::merge(&mut self.addresses, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        crate::OptionableConvert::merge(
            &mut self.deprecated_topology,
            other.deprecated_topology,
        )?;
        crate::OptionableConvert::merge(&mut self.hints, other.hints)?;
        crate::OptionableConvert::merge(&mut self.hostname, other.hostname)?;
        crate::OptionableConvert::merge(&mut self.node_name, other.node_name)?;
        crate::OptionableConvert::merge(&mut self.target_ref, other.target_ref)?;
        crate::OptionableConvert::merge(&mut self.zone, other.zone)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::discovery::v1::Endpoint> for EndpointAc {
    fn from_optionable(value: ::k8s_openapi::api::discovery::v1::Endpoint) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::discovery::v1::Endpoint, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::discovery::v1::Endpoint,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
