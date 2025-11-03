#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressClassAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: <Option<
        ::k8s_openapi::api::networking::v1::IngressClassSpec,
    > as crate::Optionable>::Optioned,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    pub phantom: std::marker::PhantomData<IngressClassAc>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::IngressClass {
    type Optioned = IngressClassAc;
}
#[automatically_derived]
impl crate::Optionable for IngressClassAc {
    type Optioned = IngressClassAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::IngressClass {
    fn into_optioned(self) -> IngressClassAc {
        IngressClassAc {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(
        value: IngressClassAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(&mut self, other: IngressClassAc) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
impl k8s_openapi::Resource for IngressClassAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::networking::v1::IngressClass as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::networking::v1::IngressClass as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::networking::v1::IngressClass as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::networking::v1::IngressClass as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::networking::v1::IngressClass as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::networking::v1::IngressClass as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for IngressClassAc {
    type Ty = <::k8s_openapi::api::networking::v1::IngressClass as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
