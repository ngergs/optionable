#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RoleBindingAc {
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
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_ref: Option<
        <::k8s_openapi::api::rbac::v1::RoleRef as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subjects: <Option<
        std::vec::Vec<::k8s_openapi::api::rbac::v1::Subject>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::rbac::v1::RoleBinding {
    type Optioned = RoleBindingAc;
}
#[automatically_derived]
impl crate::Optionable for RoleBindingAc {
    type Optioned = RoleBindingAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::rbac::v1::RoleBinding {
    fn into_optioned(self) -> RoleBindingAc {
        RoleBindingAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            role_ref: Some(crate::OptionableConvert::into_optioned(self.role_ref)),
            subjects: crate::OptionableConvert::into_optioned(self.subjects),
        }
    }
    fn try_from_optioned(value: RoleBindingAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            role_ref: crate::OptionableConvert::try_from_optioned(
                value
                    .role_ref
                    .ok_or(crate::Error {
                        missing_field: "role_ref",
                    })?,
            )?,
            subjects: crate::OptionableConvert::try_from_optioned(value.subjects)?,
        })
    }
    fn merge(&mut self, other: RoleBindingAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.role_ref {
            crate::OptionableConvert::merge(&mut self.role_ref, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.subjects, other.subjects)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::rbac::v1::RoleBinding>
for RoleBindingAc {
    fn from_optionable(value: ::k8s_openapi::api::rbac::v1::RoleBinding) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::rbac::v1::RoleBinding, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::rbac::v1::RoleBinding,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi::Resource for RoleBindingAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::rbac::v1::RoleBinding as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::rbac::v1::RoleBinding as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::rbac::v1::RoleBinding as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::rbac::v1::RoleBinding as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::rbac::v1::RoleBinding as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::rbac::v1::RoleBinding as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for RoleBindingAc {
    type Ty = <::k8s_openapi::api::rbac::v1::RoleBinding as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_rolebindingac() {
    crate::testutil::roundtrip_test::<::k8s_openapi::api::rbac::v1::RoleBinding>();
}
