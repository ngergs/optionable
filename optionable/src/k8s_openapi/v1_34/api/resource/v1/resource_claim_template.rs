pub struct ResourceClaimTemplateOpt {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub spec: Option<
        <::k8s_openapi::api::resource::v1::ResourceClaimTemplateSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::ResourceClaimTemplate {
    type Optioned = ResourceClaimTemplateOpt;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimTemplateOpt {
    type Optioned = ResourceClaimTemplateOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1::ResourceClaimTemplate {
    fn into_optioned(self) -> ResourceClaimTemplateOpt {
        ResourceClaimTemplateOpt {
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
        }
    }
    fn try_from_optioned(
        value: ResourceClaimTemplateOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::optionable::Error {
                        missing_field: "spec",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceClaimTemplateOpt,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        Ok(())
    }
}
