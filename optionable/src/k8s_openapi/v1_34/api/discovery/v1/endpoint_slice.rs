#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct EndpointSliceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::discovery::v1::Endpoint,
        > as crate::Optionable>::Optioned,
    >,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
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
impl k8s_openapi::Resource for EndpointSliceAc {
    const API_VERSION: &'static str = "discovery.k8s.io/v1";
    const GROUP: &'static str = "discovery.k8s.io";
    const KIND: &'static str = "EndpointSlice";
    const VERSION: &'static str = "v1";
    const URL_PATH_SEGMENT: &'static str = "endpointslices";
    type Scope = k8s_openapi::NamespaceResourceScope;
}
impl k8s_openapi::Metadata for EndpointSliceAc {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
