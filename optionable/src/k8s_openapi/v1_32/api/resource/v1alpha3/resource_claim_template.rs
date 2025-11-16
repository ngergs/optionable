#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimTemplateAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi::api::resource::v1alpha3::ResourceClaimTemplateSpec as crate::Optionable>::Optioned,
    >,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        deserialize_with = "crate::k8s_openapi::deserialize_api_envelope"
    )]
    pub phantom: std::marker::PhantomData<Self>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1alpha3::ResourceClaimTemplate {
    type Optioned = ResourceClaimTemplateAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimTemplateAc {
    type Optioned = ResourceClaimTemplateAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha3::ResourceClaimTemplate {
    fn into_optioned(self) -> ResourceClaimTemplateAc {
        ResourceClaimTemplateAc {
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(value: ResourceClaimTemplateAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::Error {
                        missing_field: "spec",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: ResourceClaimTemplateAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        Ok(())
    }
}
impl k8s_openapi::Resource for ResourceClaimTemplateAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::resource::v1alpha3::ResourceClaimTemplate as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::resource::v1alpha3::ResourceClaimTemplate as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::resource::v1alpha3::ResourceClaimTemplate as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::resource::v1alpha3::ResourceClaimTemplate as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::resource::v1alpha3::ResourceClaimTemplate as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::resource::v1alpha3::ResourceClaimTemplate as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for ResourceClaimTemplateAc {
    type Ty = <::k8s_openapi::api::resource::v1alpha3::ResourceClaimTemplate as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
