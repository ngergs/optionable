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
            metadata: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
            > as crate::OptionableConvert>::into_optioned(self.metadata),
            spec: <Option<
                ::k8s_openapi::api::batch::v1::JobSpec,
            > as crate::OptionableConvert>::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(
        value: JobTemplateSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
            > as crate::OptionableConvert>::try_from_optioned(value.metadata)?,
            spec: <Option<
                ::k8s_openapi::api::batch::v1::JobSpec,
            > as crate::OptionableConvert>::try_from_optioned(value.spec)?,
        })
    }
    fn merge(
        &mut self,
        other: JobTemplateSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
        > as crate::OptionableConvert>::merge(&mut self.metadata, other.metadata)?;
        <Option<
            ::k8s_openapi::api::batch::v1::JobSpec,
        > as crate::OptionableConvert>::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
