pub struct EndpointAddressOpt {
    pub hostname: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub ip: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub node_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub target_ref: <Option<
        ::k8s_openapi::api::core::v1::ObjectReference,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::endpoint_address::EndpointAddress {
    type Optioned = EndpointAddressOpt;
}
#[automatically_derived]
impl crate::Optionable for EndpointAddressOpt {
    type Optioned = EndpointAddressOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::endpoint_address::EndpointAddress {
    fn into_optioned(self) -> EndpointAddressOpt {
        EndpointAddressOpt {
            hostname: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.hostname),
            ip: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(self.ip),
            ),
            node_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.node_name),
            target_ref: <Option<
                ::k8s_openapi::api::core::v1::ObjectReference,
            > as crate::OptionableConvert>::into_optioned(self.target_ref),
        }
    }
    fn try_from_optioned(
        value: EndpointAddressOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            hostname: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.hostname)?,
            ip: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .ip
                    .ok_or(crate::optionable::Error {
                        missing_field: "ip",
                    })?,
            )?,
            node_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.node_name)?,
            target_ref: <Option<
                ::k8s_openapi::api::core::v1::ObjectReference,
            > as crate::OptionableConvert>::try_from_optioned(value.target_ref)?,
        })
    }
    fn merge(
        &mut self,
        other: EndpointAddressOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.hostname, other.hostname)?;
        if let Some(other_value) = other.ip {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.ip,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.node_name, other.node_name)?;
        <Option<
            ::k8s_openapi::api::core::v1::ObjectReference,
        > as crate::OptionableConvert>::merge(&mut self.target_ref, other.target_ref)?;
        Ok(())
    }
}
