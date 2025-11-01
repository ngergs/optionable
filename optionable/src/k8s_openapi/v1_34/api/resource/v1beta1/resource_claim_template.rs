#[derive(kube::Resource)]
#[resource(inherit = ResourceClaimTemplate)]
pub struct ResourceClaimTemplateAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub spec: Option<
        <::k8s_openapi::api::resource::v1beta1::ResourceClaimTemplateSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta1::ResourceClaimTemplate {
    type Optioned = ResourceClaimTemplateAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimTemplateAc {
    type Optioned = ResourceClaimTemplateAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta1::ResourceClaimTemplate {
    fn into_optioned(self) -> ResourceClaimTemplateAc {
        ResourceClaimTemplateAc {
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
        }
    }
    fn try_from_optioned(
        value: ResourceClaimTemplateAc,
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
        other: ResourceClaimTemplateAc,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::resource::v1beta1::ResourceClaimTemplate;
