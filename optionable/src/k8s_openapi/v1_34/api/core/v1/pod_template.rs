#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    kube::Resource
)]
#[resource(inherit = PodTemplate)]
pub struct PodTemplateAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: <Option<
        ::k8s_openapi::api::core::v1::PodTemplateSpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PodTemplate {
    type Optioned = PodTemplateAc;
}
#[automatically_derived]
impl crate::Optionable for PodTemplateAc {
    type Optioned = PodTemplateAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PodTemplate {
    fn into_optioned(self) -> PodTemplateAc {
        PodTemplateAc {
            metadata: self.metadata,
            template: crate::OptionableConvert::into_optioned(self.template),
        }
    }
    fn try_from_optioned(
        value: PodTemplateAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            template: crate::OptionableConvert::try_from_optioned(value.template)?,
        })
    }
    fn merge(&mut self, other: PodTemplateAc) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.template, other.template)?;
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::core::v1::PodTemplate;
