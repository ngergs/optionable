#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimTemplateSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi::api::resource::v1alpha3::ResourceClaimSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1alpha3::ResourceClaimTemplateSpec {
    type Optioned = ResourceClaimTemplateSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimTemplateSpecAc {
    type Optioned = ResourceClaimTemplateSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha3::ResourceClaimTemplateSpec {
    fn into_optioned(self) -> ResourceClaimTemplateSpecAc {
        ResourceClaimTemplateSpecAc {
            metadata: crate::OptionableConvert::into_optioned(self.metadata),
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
        }
    }
    fn try_from_optioned(
        value: ResourceClaimTemplateSpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: crate::OptionableConvert::try_from_optioned(value.metadata)?,
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
        other: ResourceClaimTemplateSpecAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.metadata, other.metadata)?;
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        Ok(())
    }
}
