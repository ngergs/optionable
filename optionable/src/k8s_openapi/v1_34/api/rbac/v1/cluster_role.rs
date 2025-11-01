#[derive(kube::Resource)]
#[resource(inherit = ClusterRole)]
pub struct ClusterRoleAc {
    pub aggregation_rule: <Option<
        ::k8s_openapi::api::rbac::v1::AggregationRule,
    > as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub rules: <Option<
        std::vec::Vec<::k8s_openapi::api::rbac::v1::PolicyRule>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::rbac::v1::ClusterRole {
    type Optioned = ClusterRoleAc;
}
#[automatically_derived]
impl crate::Optionable for ClusterRoleAc {
    type Optioned = ClusterRoleAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::rbac::v1::ClusterRole {
    fn into_optioned(self) -> ClusterRoleAc {
        ClusterRoleAc {
            aggregation_rule: crate::OptionableConvert::into_optioned(
                self.aggregation_rule,
            ),
            metadata: self.metadata,
            rules: crate::OptionableConvert::into_optioned(self.rules),
        }
    }
    fn try_from_optioned(
        value: ClusterRoleAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            aggregation_rule: crate::OptionableConvert::try_from_optioned(
                value.aggregation_rule,
            )?,
            metadata: value.metadata,
            rules: crate::OptionableConvert::try_from_optioned(value.rules)?,
        })
    }
    fn merge(&mut self, other: ClusterRoleAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.aggregation_rule,
            other.aggregation_rule,
        )?;
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.rules, other.rules)?;
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::rbac::v1::ClusterRole;
