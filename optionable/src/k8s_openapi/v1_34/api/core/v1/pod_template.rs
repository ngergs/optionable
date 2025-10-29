pub struct PodTemplateOpt {
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
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
            metadata: Some(crate::OptionableConvert::into_optioned(self.metadata)),
            template: crate::OptionableConvert::into_optioned(self.template),
        }
    }
    fn try_from_optioned(
        value: PodTemplateOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: crate::OptionableConvert::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            template: crate::OptionableConvert::try_from_optioned(value.template)?,
        })
    }
    fn merge(&mut self, other: PodTemplateOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.metadata {
            crate::OptionableConvert::merge(&mut self.metadata, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.template, other.template)?;
        Ok(())
    }
}
