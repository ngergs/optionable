#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodTemplateSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: <Option<
        ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: <Option<
        ::k8s_openapi027::api::core::v1::PodSpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PodTemplateSpec {
    type Optioned = PodTemplateSpecAc;
}
#[automatically_derived]
impl crate::Optionable for PodTemplateSpecAc {
    type Optioned = PodTemplateSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::PodTemplateSpec {
    fn into_optioned(self) -> PodTemplateSpecAc {
        PodTemplateSpecAc {
            metadata: crate::OptionableConvert::into_optioned(self.metadata),
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(value: PodTemplateSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: crate::OptionableConvert::try_from_optioned(value.metadata)?,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(&mut self, other: PodTemplateSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.metadata, other.metadata)?;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PodTemplateSpec>
for PodTemplateSpecAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::PodTemplateSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PodTemplateSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PodTemplateSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
