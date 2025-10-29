pub struct ClusterRoleOpt {
    pub aggregation_rule: <Option<
        ::k8s_openapi::api::rbac::v1::AggregationRule,
    > as crate::Optionable>::Optioned,
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub rules: <Option<
        std::vec::Vec<::k8s_openapi::api::rbac::v1::PolicyRule>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::rbac::v1::ClusterRole {
    type Optioned = ClusterRoleOpt;
}
#[automatically_derived]
impl crate::Optionable for ClusterRoleOpt {
    type Optioned = ClusterRoleOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::rbac::v1::ClusterRole {
    fn into_optioned(self) -> ClusterRoleOpt {
        ClusterRoleOpt {
            aggregation_rule: crate::OptionableConvert::into_optioned(
                self.aggregation_rule,
            ),
            metadata: Some(crate::OptionableConvert::into_optioned(self.metadata)),
            rules: crate::OptionableConvert::into_optioned(self.rules),
        }
    }
    fn try_from_optioned(
        value: ClusterRoleOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            aggregation_rule: crate::OptionableConvert::try_from_optioned(
                value.aggregation_rule,
            )?,
            metadata: crate::OptionableConvert::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            rules: crate::OptionableConvert::try_from_optioned(value.rules)?,
        })
    }
    fn merge(&mut self, other: ClusterRoleOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.aggregation_rule,
            other.aggregation_rule,
        )?;
        if let Some(other_value) = other.metadata {
            crate::OptionableConvert::merge(&mut self.metadata, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.rules, other.rules)?;
        Ok(())
    }
}
