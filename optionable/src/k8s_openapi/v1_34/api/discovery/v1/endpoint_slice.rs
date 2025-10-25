pub struct EndpointSliceOpt {
    pub address_type: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub endpoints: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::discovery::v1::Endpoint,
        > as crate::Optionable>::Optioned,
    >,
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub ports: <Option<
        std::vec::Vec<::k8s_openapi::api::discovery::v1::EndpointPort>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::discovery::v1::endpoint_slice::EndpointSlice {
    type Optioned = EndpointSliceOpt;
}
#[automatically_derived]
impl crate::Optionable for EndpointSliceOpt {
    type Optioned = EndpointSliceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::discovery::v1::endpoint_slice::EndpointSlice {
    fn into_optioned(self) -> EndpointSliceOpt {
        EndpointSliceOpt {
            address_type: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.address_type,
                ),
            ),
            endpoints: Some(
                <std::vec::Vec<
                    ::k8s_openapi::api::discovery::v1::Endpoint,
                > as crate::OptionableConvert>::into_optioned(self.endpoints),
            ),
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            ports: <Option<
                std::vec::Vec<::k8s_openapi::api::discovery::v1::EndpointPort>,
            > as crate::OptionableConvert>::into_optioned(self.ports),
        }
    }
    fn try_from_optioned(
        value: EndpointSliceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            address_type: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .address_type
                    .ok_or(crate::optionable::Error {
                        missing_field: "address_type",
                    })?,
            )?,
            endpoints: <std::vec::Vec<
                ::k8s_openapi::api::discovery::v1::Endpoint,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .endpoints
                    .ok_or(crate::optionable::Error {
                        missing_field: "endpoints",
                    })?,
            )?,
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            ports: <Option<
                std::vec::Vec<::k8s_openapi::api::discovery::v1::EndpointPort>,
            > as crate::OptionableConvert>::try_from_optioned(value.ports)?,
        })
    }
    fn merge(
        &mut self,
        other: EndpointSliceOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.address_type {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.address_type,
                other_value,
            )?;
        }
        if let Some(other_value) = other.endpoints {
            <std::vec::Vec<
                ::k8s_openapi::api::discovery::v1::Endpoint,
            > as crate::OptionableConvert>::merge(&mut self.endpoints, other_value)?;
        }
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        <Option<
            std::vec::Vec<::k8s_openapi::api::discovery::v1::EndpointPort>,
        > as crate::OptionableConvert>::merge(&mut self.ports, other.ports)?;
        Ok(())
    }
}
