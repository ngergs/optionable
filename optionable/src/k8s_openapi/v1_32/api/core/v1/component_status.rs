#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ComponentStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ComponentCondition>,
    > as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    pub phantom: std::marker::PhantomData<ComponentStatusAc>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ComponentStatus {
    type Optioned = ComponentStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ComponentStatusAc {
    type Optioned = ComponentStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ComponentStatus {
    fn into_optioned(self) -> ComponentStatusAc {
        ComponentStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            metadata: self.metadata,
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(
        value: ComponentStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            metadata: value.metadata,
        })
    }
    fn merge(
        &mut self,
        other: ComponentStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        self.metadata = other.metadata;
        Ok(())
    }
}
impl k8s_openapi::Resource for ComponentStatusAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::core::v1::ComponentStatus as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::core::v1::ComponentStatus as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::core::v1::ComponentStatus as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::core::v1::ComponentStatus as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::core::v1::ComponentStatus as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::core::v1::ComponentStatus as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for ComponentStatusAc {
    type Ty = <::k8s_openapi::api::core::v1::ComponentStatus as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
