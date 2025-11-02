#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    kube::Resource
)]
#[resource(inherit = Endpoints)]
pub struct EndpointsAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsets: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::EndpointSubset>,
    > as crate::Optionable>::Optioned,
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
#[allow(unused_imports)]
use ::k8s_openapi::api::core::v1::Endpoints;
