pub struct JobTemplateSpecOpt {
    pub metadata: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    > as crate::Optionable>::Optioned,
    pub spec: <Option<
        ::k8s_openapi::api::batch::v1::JobSpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::batch::v1::JobTemplateSpec {
    type Optioned = JobTemplateSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for JobTemplateSpecOpt {
    type Optioned = JobTemplateSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::batch::v1::JobTemplateSpec {
    fn into_optioned(self) -> JobTemplateSpecOpt {
        JobTemplateSpecOpt {
            metadata: crate::OptionableConvert::into_optioned(self.metadata),
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(
        value: JobTemplateSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: crate::OptionableConvert::try_from_optioned(value.metadata)?,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(
        &mut self,
        other: JobTemplateSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.metadata, other.metadata)?;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
