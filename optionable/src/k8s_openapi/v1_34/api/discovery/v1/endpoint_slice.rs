#[derive(kube::Resource)]
#[resource(inherit = EndpointSlice)]
pub struct EndpointSliceAc {
    pub address_type: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub endpoints: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::discovery::v1::Endpoint,
        > as crate::Optionable>::Optioned,
    >,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub ports: <Option<
        std::vec::Vec<::k8s_openapi::api::discovery::v1::EndpointPort>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::discovery::v1::EndpointSlice {
    type Optioned = EndpointSliceAc;
}
#[automatically_derived]
impl crate::Optionable for EndpointSliceAc {
    type Optioned = EndpointSliceAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::discovery::v1::EndpointSlice {
    fn into_optioned(self) -> EndpointSliceAc {
        EndpointSliceAc {
            address_type: Some(
                crate::OptionableConvert::into_optioned(self.address_type),
            ),
            endpoints: Some(crate::OptionableConvert::into_optioned(self.endpoints)),
            metadata: self.metadata,
            ports: crate::OptionableConvert::into_optioned(self.ports),
        }
    }
    fn try_from_optioned(
        value: EndpointSliceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            address_type: crate::OptionableConvert::try_from_optioned(
                value
                    .address_type
                    .ok_or(crate::optionable::Error {
                        missing_field: "address_type",
                    })?,
            )?,
            endpoints: crate::OptionableConvert::try_from_optioned(
                value
                    .endpoints
                    .ok_or(crate::optionable::Error {
                        missing_field: "endpoints",
                    })?,
            )?,
            metadata: value.metadata,
            ports: crate::OptionableConvert::try_from_optioned(value.ports)?,
        })
    }
    fn merge(&mut self, other: EndpointSliceAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.address_type {
            crate::OptionableConvert::merge(&mut self.address_type, other_value)?;
        }
        if let Some(other_value) = other.endpoints {
            crate::OptionableConvert::merge(&mut self.endpoints, other_value)?;
        }
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.ports, other.ports)?;
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::discovery::v1::EndpointSlice;
