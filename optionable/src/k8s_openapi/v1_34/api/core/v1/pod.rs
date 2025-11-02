#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    kube::Resource
)]
#[resource(inherit = Pod)]
pub struct PodAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: <Option<
        ::k8s_openapi::api::core::v1::PodSpec,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: <Option<
        ::k8s_openapi::api::core::v1::PodStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::Pod {
    type Optioned = PodAc;
}
#[automatically_derived]
impl crate::Optionable for PodAc {
    type Optioned = PodAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Pod {
    fn into_optioned(self) -> PodAc {
        PodAc {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(value: PodAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(&mut self, other: PodAc) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::core::v1::Pod;
