#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct JobTemplateSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: <Option<
        ::k8s_openapi026::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: <Option<
        ::k8s_openapi026::api::batch::v1::JobSpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::batch::v1::JobTemplateSpec {
    type Optioned = JobTemplateSpecAc;
}
#[automatically_derived]
impl crate::Optionable for JobTemplateSpecAc {
    type Optioned = JobTemplateSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::batch::v1::JobTemplateSpec {
    fn into_optioned(self) -> JobTemplateSpecAc {
        JobTemplateSpecAc {
            metadata: crate::OptionableConvert::into_optioned(self.metadata),
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(value: JobTemplateSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: crate::OptionableConvert::try_from_optioned(value.metadata)?,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(&mut self, other: JobTemplateSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.metadata, other.metadata)?;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::batch::v1::JobTemplateSpec>
for JobTemplateSpecAc {
    fn from_optionable(value: k8s_openapi026::api::batch::v1::JobTemplateSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::batch::v1::JobTemplateSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::batch::v1::JobTemplateSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
