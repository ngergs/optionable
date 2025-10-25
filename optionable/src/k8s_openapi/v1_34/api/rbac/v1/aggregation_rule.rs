pub struct AggregationRuleOpt {
    pub cluster_role_selectors: <Option<
        std::vec::Vec<::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::rbac::v1::aggregation_rule::AggregationRule {
    type Optioned = AggregationRuleOpt;
}
#[automatically_derived]
impl crate::Optionable for AggregationRuleOpt {
    type Optioned = AggregationRuleOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::rbac::v1::aggregation_rule::AggregationRule {
    fn into_optioned(self) -> AggregationRuleOpt {
        AggregationRuleOpt {
            cluster_role_selectors: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
                >,
            > as crate::OptionableConvert>::into_optioned(self.cluster_role_selectors),
        }
    }
    fn try_from_optioned(
        value: AggregationRuleOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            cluster_role_selectors: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
                >,
            > as crate::OptionableConvert>::try_from_optioned(
                value.cluster_role_selectors,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: AggregationRuleOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.cluster_role_selectors,
            other.cluster_role_selectors,
        )?;
        Ok(())
    }
}
