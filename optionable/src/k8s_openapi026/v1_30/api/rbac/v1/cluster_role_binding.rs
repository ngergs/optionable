#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClusterRoleBindingAc {
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
    pub metadata: ::k8s_openapi026::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_ref: Option<
        <::k8s_openapi026::api::rbac::v1::RoleRef as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subjects: <Option<
        std::vec::Vec<::k8s_openapi026::api::rbac::v1::Subject>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::rbac::v1::ClusterRoleBinding {
    type Optioned = ClusterRoleBindingAc;
}
#[automatically_derived]
impl crate::Optionable for ClusterRoleBindingAc {
    type Optioned = ClusterRoleBindingAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::rbac::v1::ClusterRoleBinding {
    fn into_optioned(self) -> ClusterRoleBindingAc {
        ClusterRoleBindingAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            role_ref: Some(crate::OptionableConvert::into_optioned(self.role_ref)),
            subjects: crate::OptionableConvert::into_optioned(self.subjects),
        }
    }
    fn try_from_optioned(value: ClusterRoleBindingAc) -> Result<Self, crate::Error> {
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
    fn merge(&mut self, other: ClusterRoleBindingAc) -> Result<(), crate::Error> {
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
impl crate::OptionedConvert<k8s_openapi026::api::rbac::v1::ClusterRoleBinding>
for ClusterRoleBindingAc {
    fn from_optionable(
        value: k8s_openapi026::api::rbac::v1::ClusterRoleBinding,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::rbac::v1::ClusterRoleBinding, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::rbac::v1::ClusterRoleBinding,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi026::Resource for ClusterRoleBindingAc {
    const API_VERSION: &'static str = <k8s_openapi026::api::rbac::v1::ClusterRoleBinding as k8s_openapi026::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi026::api::rbac::v1::ClusterRoleBinding as k8s_openapi026::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi026::api::rbac::v1::ClusterRoleBinding as k8s_openapi026::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi026::api::rbac::v1::ClusterRoleBinding as k8s_openapi026::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi026::api::rbac::v1::ClusterRoleBinding as k8s_openapi026::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi026::api::rbac::v1::ClusterRoleBinding as k8s_openapi026::Resource>::Scope;
}
impl k8s_openapi026::Metadata for ClusterRoleBindingAc {
    type Ty = <k8s_openapi026::api::rbac::v1::ClusterRoleBinding as k8s_openapi026::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi026::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi026::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_clusterrolebindingac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi026::api::rbac::v1::ClusterRoleBinding,
    >();
}
