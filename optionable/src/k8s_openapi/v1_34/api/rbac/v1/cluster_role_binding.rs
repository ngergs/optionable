#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterRoleBindingAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_ref: Option<
        <::k8s_openapi::api::rbac::v1::RoleRef as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subjects: <Option<
        std::vec::Vec<::k8s_openapi::api::rbac::v1::Subject>,
    > as crate::Optionable>::Optioned,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    pub phantom: std::marker::PhantomData<ClusterRoleBindingAc>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::rbac::v1::ClusterRoleBinding {
    type Optioned = ClusterRoleBindingAc;
}
#[automatically_derived]
impl crate::Optionable for ClusterRoleBindingAc {
    type Optioned = ClusterRoleBindingAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::rbac::v1::ClusterRoleBinding {
    fn into_optioned(self) -> ClusterRoleBindingAc {
        ClusterRoleBindingAc {
            metadata: self.metadata,
            role_ref: Some(crate::OptionableConvert::into_optioned(self.role_ref)),
            subjects: crate::OptionableConvert::into_optioned(self.subjects),
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(
        value: ClusterRoleBindingAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            role_ref: crate::OptionableConvert::try_from_optioned(
                value
                    .role_ref
                    .ok_or(crate::optionable::Error {
                        missing_field: "role_ref",
                    })?,
            )?,
            subjects: crate::OptionableConvert::try_from_optioned(value.subjects)?,
        })
    }
    fn merge(
        &mut self,
        other: ClusterRoleBindingAc,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.role_ref {
            crate::OptionableConvert::merge(&mut self.role_ref, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.subjects, other.subjects)?;
        Ok(())
    }
}
impl k8s_openapi::Resource for ClusterRoleBindingAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::rbac::v1::ClusterRoleBinding as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::rbac::v1::ClusterRoleBinding as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::rbac::v1::ClusterRoleBinding as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::rbac::v1::ClusterRoleBinding as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::rbac::v1::ClusterRoleBinding as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::rbac::v1::ClusterRoleBinding as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for ClusterRoleBindingAc {
    type Ty = <::k8s_openapi::api::rbac::v1::ClusterRoleBinding as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
