#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// EndpointSlice represents a subset of the endpoints that implement a service. For a given service there may be multiple EndpointSlice objects, selected by labels, which must be joined to produce the full set of endpoints.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EndpointSliceAc {
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_api_version",
        deserialize_with = "crate::k8s_openapi::deserialize_api_version"
    )]
    pub api_version: std::marker::PhantomData<Self>,
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_kind",
        deserialize_with = "crate::k8s_openapi::deserialize_kind"
    )]
    pub kind: std::marker::PhantomData<Self>,
    /// addressType specifies the type of address carried by this EndpointSlice. All addresses in this slice must be the same type. This field is immutable after creation. The following address types are currently supported: * IPv4: Represents an IPv4 Address. * IPv6: Represents an IPv6 Address. * FQDN: Represents a Fully Qualified Domain Name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<std::string::String>,
    /// endpoints is a list of unique endpoints in this slice. Each slice may include a maximum of 1000 endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::discovery::v1::Endpoint as crate::Optionable>::Optioned,
        >,
    >,
    /// Standard object's metadata.
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// ports specifies the list of network ports exposed by each endpoint in this slice. Each port must have a unique name. When ports is empty, it indicates that there are no defined ports. When a port is defined with a nil port value, it indicates "all ports". Each slice may include a maximum of 100 ports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::discovery::v1::EndpointPort as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::discovery::v1::EndpointSlice {
    type Optioned = EndpointSliceAc;
}
#[automatically_derived]
impl crate::Optionable for EndpointSliceAc {
    type Optioned = EndpointSliceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::discovery::v1::EndpointSlice {
    fn into_optioned(self) -> EndpointSliceAc {
        EndpointSliceAc {
            api_version: Default::default(),
            kind: Default::default(),
            address_type: Some(self.address_type),
            endpoints: Some(crate::OptionableConvert::into_optioned(self.endpoints)),
            metadata: self.metadata,
            ports: crate::OptionableConvert::into_optioned(self.ports),
        }
    }
    fn try_from_optioned(value: EndpointSliceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            address_type: value
                .address_type
                .ok_or(crate::Error {
                    missing_field: "address_type",
                })?,
            endpoints: crate::OptionableConvert::try_from_optioned(
                value
                    .endpoints
                    .ok_or(crate::Error {
                        missing_field: "endpoints",
                    })?,
            )?,
            metadata: value.metadata,
            ports: crate::OptionableConvert::try_from_optioned(value.ports)?,
        })
    }
    fn merge(&mut self, other: EndpointSliceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.address_type {
            self.address_type = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if let Some(other_value) = other.endpoints {
            self.endpoints = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        self.metadata = other.metadata;
        if self.ports.is_none() {
            self.ports = crate::OptionableConvert::try_from_optioned(other.ports)?;
        } else if let Some(self_value) = self.ports.as_mut()
            && let Some(other_value) = other.ports
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::discovery::v1::EndpointSlice>
for EndpointSliceAc {
    fn from_optionable(
        value: k8s_openapi027::api::discovery::v1::EndpointSlice,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::discovery::v1::EndpointSlice, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::discovery::v1::EndpointSlice,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for EndpointSliceAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::discovery::v1::EndpointSlice as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::discovery::v1::EndpointSlice as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::discovery::v1::EndpointSlice as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::discovery::v1::EndpointSlice as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::discovery::v1::EndpointSlice as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::discovery::v1::EndpointSlice as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for EndpointSliceAc {
    type Ty = <k8s_openapi027::api::discovery::v1::EndpointSlice as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_endpointsliceac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi027::api::discovery::v1::EndpointSlice,
    >();
}
impl k8s_openapi027::DeepMerge for EndpointSliceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.address_type,
            other.address_type,
        );
        self.endpoints = other.endpoints;
        k8s_openapi027::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        self.ports = other.ports;
    }
}
