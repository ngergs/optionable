#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ComponentStatusAc {
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_api_version",
        deserialize_with = "crate::k8s_openapi::deserialize_api_version"
    )]
    pub api_version: std::marker::PhantomData<Self>,
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_kind",
        deserialize_with = "crate::k8s_openapi::deserialize_kind"
    )]
    pub kind: std::marker::PhantomData<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi027::api::core::v1::ComponentCondition>,
    > as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ComponentStatus {
    type Optioned = ComponentStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ComponentStatusAc {
    type Optioned = ComponentStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ComponentStatus {
    fn into_optioned(self) -> ComponentStatusAc {
        ComponentStatusAc {
            api_version: Default::default(),
            kind: Default::default(),
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            metadata: self.metadata,
        }
    }
    fn try_from_optioned(value: ComponentStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            metadata: value.metadata,
        })
    }
    fn merge(&mut self, other: ComponentStatusAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        self.metadata = other.metadata;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ComponentStatus>
for ComponentStatusAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::ComponentStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ComponentStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ComponentStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for ComponentStatusAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::core::v1::ComponentStatus as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::core::v1::ComponentStatus as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::core::v1::ComponentStatus as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::core::v1::ComponentStatus as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::core::v1::ComponentStatus as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::core::v1::ComponentStatus as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for ComponentStatusAc {
    type Ty = <k8s_openapi027::api::core::v1::ComponentStatus as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_componentstatusac() {
    crate::testutil::roundtrip_test::<k8s_openapi027::api::core::v1::ComponentStatus>();
}
