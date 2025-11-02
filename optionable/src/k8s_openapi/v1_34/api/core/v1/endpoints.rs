#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointsAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsets: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::EndpointSubset>,
    > as crate::Optionable>::Optioned,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    phantom: std::marker::PhantomData<EndpointsAc>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::Endpoints {
    type Optioned = EndpointsAc;
}
#[automatically_derived]
impl crate::Optionable for EndpointsAc {
    type Optioned = EndpointsAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Endpoints {
    fn into_optioned(self) -> EndpointsAc {
        EndpointsAc {
            metadata: self.metadata,
            subsets: crate::OptionableConvert::into_optioned(self.subsets),
        }
    }
    fn try_from_optioned(value: EndpointsAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            subsets: crate::OptionableConvert::try_from_optioned(value.subsets)?,
        })
    }
    fn merge(&mut self, other: EndpointsAc) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.subsets, other.subsets)?;
        Ok(())
    }
}
impl k8s_openapi::Resource for EndpointsAc {
    const API_VERSION: &'static str = "v1";
    const GROUP: &'static str = "";
    const KIND: &'static str = "Endpoints";
    const VERSION: &'static str = "v1";
    const URL_PATH_SEGMENT: &'static str = "endpoints";
    type Scope = k8s_openapi::NamespaceResourceScope;
}
impl k8s_openapi::Metadata for EndpointsAc {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
