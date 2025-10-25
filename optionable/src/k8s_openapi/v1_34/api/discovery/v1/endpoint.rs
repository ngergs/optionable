pub struct EndpointOpt {
    pub addresses: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
    pub conditions: <Option<
        ::k8s_openapi::api::discovery::v1::EndpointConditions,
    > as crate::Optionable>::Optioned,
    pub deprecated_topology: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    pub hints: <Option<
        ::k8s_openapi::api::discovery::v1::EndpointHints,
    > as crate::Optionable>::Optioned,
    pub hostname: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub node_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub target_ref: <Option<
        ::k8s_openapi::api::core::v1::ObjectReference,
    > as crate::Optionable>::Optioned,
    pub zone: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::discovery::v1::endpoint::Endpoint {
    type Optioned = EndpointOpt;
}
#[automatically_derived]
impl crate::Optionable for EndpointOpt {
    type Optioned = EndpointOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::discovery::v1::endpoint::Endpoint {
    fn into_optioned(self) -> EndpointOpt {
        EndpointOpt {
            addresses: Some(
                <std::vec::Vec<
                    std::string::String,
                > as crate::OptionableConvert>::into_optioned(self.addresses),
            ),
            conditions: <Option<
                ::k8s_openapi::api::discovery::v1::EndpointConditions,
            > as crate::OptionableConvert>::into_optioned(self.conditions),
            deprecated_topology: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.deprecated_topology),
            hints: <Option<
                ::k8s_openapi::api::discovery::v1::EndpointHints,
            > as crate::OptionableConvert>::into_optioned(self.hints),
            hostname: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.hostname),
            node_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.node_name),
            target_ref: <Option<
                ::k8s_openapi::api::core::v1::ObjectReference,
            > as crate::OptionableConvert>::into_optioned(self.target_ref),
            zone: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.zone),
        }
    }
    fn try_from_optioned(value: EndpointOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            addresses: <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .addresses
                    .ok_or(crate::optionable::Error {
                        missing_field: "addresses",
                    })?,
            )?,
            conditions: <Option<
                ::k8s_openapi::api::discovery::v1::EndpointConditions,
            > as crate::OptionableConvert>::try_from_optioned(value.conditions)?,
            deprecated_topology: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(
                value.deprecated_topology,
            )?,
            hints: <Option<
                ::k8s_openapi::api::discovery::v1::EndpointHints,
            > as crate::OptionableConvert>::try_from_optioned(value.hints)?,
            hostname: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.hostname)?,
            node_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.node_name)?,
            target_ref: <Option<
                ::k8s_openapi::api::core::v1::ObjectReference,
            > as crate::OptionableConvert>::try_from_optioned(value.target_ref)?,
            zone: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.zone)?,
        })
    }
    fn merge(&mut self, other: EndpointOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.addresses {
            <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::merge(&mut self.addresses, other_value)?;
        }
        <Option<
            ::k8s_openapi::api::discovery::v1::EndpointConditions,
        > as crate::OptionableConvert>::merge(&mut self.conditions, other.conditions)?;
        <Option<
            std::collections::BTreeMap<std::string::String, std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.deprecated_topology,
            other.deprecated_topology,
        )?;
        <Option<
            ::k8s_openapi::api::discovery::v1::EndpointHints,
        > as crate::OptionableConvert>::merge(&mut self.hints, other.hints)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.hostname, other.hostname)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.node_name, other.node_name)?;
        <Option<
            ::k8s_openapi::api::core::v1::ObjectReference,
        > as crate::OptionableConvert>::merge(&mut self.target_ref, other.target_ref)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.zone, other.zone)?;
        Ok(())
    }
}
