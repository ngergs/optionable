#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClusterRoleAc {
    #[serde(
        serialize_with = "crate::k8s_openapi026::serialize_api_version",
        deserialize_with = "crate::k8s_openapi026::deserialize_api_version"
    )]
    pub api_version: std::marker::PhantomData<Self>,
    #[serde(
        serialize_with = "crate::k8s_openapi026::serialize_kind",
        deserialize_with = "crate::k8s_openapi026::deserialize_kind"
    )]
    pub kind: std::marker::PhantomData<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_rule: <Option<
        ::k8s_openapi026::api::rbac::v1::AggregationRule,
    > as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi026::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: <Option<
        std::vec::Vec<::k8s_openapi026::api::rbac::v1::PolicyRule>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::rbac::v1::ClusterRole {
    type Optioned = ClusterRoleAc;
}
#[automatically_derived]
impl crate::Optionable for ClusterRoleAc {
    type Optioned = ClusterRoleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::rbac::v1::ClusterRole {
    fn into_optioned(self) -> ClusterRoleAc {
        ClusterRoleAc {
            api_version: Default::default(),
            kind: Default::default(),
            aggregation_rule: crate::OptionableConvert::into_optioned(
                self.aggregation_rule,
            ),
            metadata: self.metadata,
            rules: crate::OptionableConvert::into_optioned(self.rules),
        }
    }
    fn try_from_optioned(value: ClusterRoleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            aggregation_rule: crate::OptionableConvert::try_from_optioned(
                value.aggregation_rule,
            )?,
            metadata: value.metadata,
            rules: crate::OptionableConvert::try_from_optioned(value.rules)?,
        })
    }
    fn merge(&mut self, other: ClusterRoleAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.aggregation_rule,
            other.aggregation_rule,
        )?;
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.rules, other.rules)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::rbac::v1::ClusterRole>
for ClusterRoleAc {
    fn from_optionable(value: k8s_openapi026::api::rbac::v1::ClusterRole) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::rbac::v1::ClusterRole, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::rbac::v1::ClusterRole,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi026::Resource for ClusterRoleAc {
    const API_VERSION: &'static str = <k8s_openapi026::api::rbac::v1::ClusterRole as k8s_openapi026::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi026::api::rbac::v1::ClusterRole as k8s_openapi026::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi026::api::rbac::v1::ClusterRole as k8s_openapi026::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi026::api::rbac::v1::ClusterRole as k8s_openapi026::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi026::api::rbac::v1::ClusterRole as k8s_openapi026::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi026::api::rbac::v1::ClusterRole as k8s_openapi026::Resource>::Scope;
}
impl k8s_openapi026::Metadata for ClusterRoleAc {
    type Ty = <k8s_openapi026::api::rbac::v1::ClusterRole as k8s_openapi026::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi026::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi026::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_clusterroleac() {
    crate::testutil::roundtrip_test::<k8s_openapi026::api::rbac::v1::ClusterRole>();
}
