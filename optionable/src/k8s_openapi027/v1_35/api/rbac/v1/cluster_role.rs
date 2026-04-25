#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ClusterRole is a cluster level, logical grouping of PolicyRules that can be referenced as a unit by a RoleBinding or ClusterRoleBinding.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClusterRoleAc {
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
    /// AggregationRule is an optional field that describes how to build the Rules for this ClusterRole. If AggregationRule is set, then the Rules are controller managed and direct changes to Rules will be stomped by the controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_rule: Option<
        <::k8s_openapi027::api::rbac::v1::AggregationRule as crate::Optionable>::Optioned,
    >,
    /// Standard object's metadata.
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// Rules holds all the PolicyRules for this ClusterRole
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::rbac::v1::PolicyRule as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::rbac::v1::ClusterRole {
    type Optioned = ClusterRoleAc;
}
#[automatically_derived]
impl crate::Optionable for ClusterRoleAc {
    type Optioned = ClusterRoleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::rbac::v1::ClusterRole {
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
        if self.aggregation_rule.is_none() {
            self.aggregation_rule = crate::OptionableConvert::try_from_optioned(
                other.aggregation_rule,
            )?;
        } else if let Some(self_value) = self.aggregation_rule.as_mut()
            && let Some(other_value) = other.aggregation_rule
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        self.metadata = other.metadata;
        if self.rules.is_none() {
            self.rules = crate::OptionableConvert::try_from_optioned(other.rules)?;
        } else if let Some(self_value) = self.rules.as_mut()
            && let Some(other_value) = other.rules
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::rbac::v1::ClusterRole>
for ClusterRoleAc {
    fn from_optionable(value: k8s_openapi027::api::rbac::v1::ClusterRole) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::rbac::v1::ClusterRole, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::rbac::v1::ClusterRole,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for ClusterRoleAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::rbac::v1::ClusterRole as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::rbac::v1::ClusterRole as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::rbac::v1::ClusterRole as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::rbac::v1::ClusterRole as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::rbac::v1::ClusterRole as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::rbac::v1::ClusterRole as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for ClusterRoleAc {
    type Ty = <k8s_openapi027::api::rbac::v1::ClusterRole as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_clusterroleac() {
    crate::testutil::roundtrip_test::<k8s_openapi027::api::rbac::v1::ClusterRole>();
}
impl k8s_openapi027::DeepMerge for ClusterRoleAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.api_version, other.api_version);
        k8s_openapi027::DeepMerge::merge_from(&mut self.kind, other.kind);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.aggregation_rule,
            other.aggregation_rule,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        self.rules = other.rules;
    }
}
