#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// RoleBinding references a role, but does not contain it.  It can reference a Role in the same namespace or a ClusterRole in the global namespace. It adds who information via Subjects and namespace information by which namespace it exists in.  RoleBindings in a given namespace only have effect in that namespace.
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
    /// Standard object's metadata.
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// RoleRef can reference a Role in the current namespace or a ClusterRole in the global namespace. If the RoleRef cannot be resolved, the Authorizer must return an error. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_ref: Option<
        <::k8s_openapi027::api::rbac::v1::RoleRef as crate::Optionable>::Optioned,
    >,
    /// Subjects holds references to the objects the role applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subjects: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::rbac::v1::Subject as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::rbac::v1::RoleBinding {
    type Optioned = RoleBindingAc;
}
#[automatically_derived]
impl crate::Optionable for RoleBindingAc {
    type Optioned = RoleBindingAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::rbac::v1::RoleBinding {
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
            self.role_ref = other_value;
        }
        if self.subjects.is_none() {
            self.subjects = other.subjects;
        }
        if let Some(other_value) = other.subjects {
            self.subjects = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::rbac::v1::RoleBinding>
for RoleBindingAc {
    fn from_optionable(value: k8s_openapi027::api::rbac::v1::RoleBinding) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::rbac::v1::RoleBinding, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::rbac::v1::RoleBinding,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for RoleBindingAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::rbac::v1::RoleBinding as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::rbac::v1::RoleBinding as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::rbac::v1::RoleBinding as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::rbac::v1::RoleBinding as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::rbac::v1::RoleBinding as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::rbac::v1::RoleBinding as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for RoleBindingAc {
    type Ty = <k8s_openapi027::api::rbac::v1::RoleBinding as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_rolebindingac() {
    crate::testutil::roundtrip_test::<k8s_openapi027::api::rbac::v1::RoleBinding>();
}
