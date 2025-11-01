pub struct PodTemplateOpt {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub template: <Option<
        ::k8s_openapi::api::core::v1::PodTemplateSpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PodTemplate {
    type Optioned = PodTemplateOpt;
}
#[automatically_derived]
impl crate::Optionable for PodTemplateOpt {
    type Optioned = PodTemplateOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PodTemplate {
    fn into_optioned(self) -> PodTemplateOpt {
        PodTemplateOpt {
            metadata: self.metadata,
            template: crate::OptionableConvert::into_optioned(self.template),
        }
    }
    fn try_from_optioned(
        value: PodTemplateOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            template: crate::OptionableConvert::try_from_optioned(value.template)?,
        })
    }
    fn merge(&mut self, other: PodTemplateOpt) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.template, other.template)?;
        Ok(())
    }
}
