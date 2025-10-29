pub struct PodTemplateSpecOpt {
    pub metadata: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    > as crate::Optionable>::Optioned,
    pub spec: <Option<
        ::k8s_openapi::api::core::v1::PodSpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PodTemplateSpec {
    type Optioned = PodTemplateSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for PodTemplateSpecOpt {
    type Optioned = PodTemplateSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PodTemplateSpec {
    fn into_optioned(self) -> PodTemplateSpecOpt {
        PodTemplateSpecOpt {
            metadata: crate::OptionableConvert::into_optioned(self.metadata),
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(
        value: PodTemplateSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: crate::OptionableConvert::try_from_optioned(value.metadata)?,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(
        &mut self,
        other: PodTemplateSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.metadata, other.metadata)?;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
