#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct RoleAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: <Option<
        std::vec::Vec<::k8s_openapi::api::rbac::v1::PolicyRule>,
    > as crate::Optionable>::Optioned,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        deserialize_with = "crate::k8s_openapi::deserialize_api_envelope"
    )]
    pub phantom: std::marker::PhantomData<Self>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::rbac::v1::Role {
    type Optioned = RoleAc;
}
#[automatically_derived]
impl crate::Optionable for RoleAc {
    type Optioned = RoleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::rbac::v1::Role {
    fn into_optioned(self) -> RoleAc {
        RoleAc {
            metadata: self.metadata,
            rules: crate::OptionableConvert::into_optioned(self.rules),
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(value: RoleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            rules: crate::OptionableConvert::try_from_optioned(value.rules)?,
        })
    }
    fn merge(&mut self, other: RoleAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.rules, other.rules)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::rbac::v1::Role> for RoleAc {
    fn from_optionable(value: ::k8s_openapi::api::rbac::v1::Role) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::rbac::v1::Role, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::rbac::v1::Role,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi::Resource for RoleAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::rbac::v1::Role as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::rbac::v1::Role as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::rbac::v1::Role as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::rbac::v1::Role as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::rbac::v1::Role as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::rbac::v1::Role as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for RoleAc {
    type Ty = <::k8s_openapi::api::rbac::v1::Role as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_roleac() {
    crate::testutil::roundtrip_test::<::k8s_openapi::api::rbac::v1::Role>();
}
