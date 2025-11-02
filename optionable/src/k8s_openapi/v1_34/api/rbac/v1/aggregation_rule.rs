#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AggregationRuleAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_role_selectors: <Option<
        std::vec::Vec<::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::rbac::v1::AggregationRule {
    type Optioned = AggregationRuleAc;
}
#[automatically_derived]
impl crate::Optionable for AggregationRuleAc {
    type Optioned = AggregationRuleAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::rbac::v1::AggregationRule {
    fn into_optioned(self) -> AggregationRuleAc {
        AggregationRuleAc {
            cluster_role_selectors: crate::OptionableConvert::into_optioned(
                self.cluster_role_selectors,
            ),
        }
    }
    fn try_from_optioned(
        value: AggregationRuleAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            cluster_role_selectors: crate::OptionableConvert::try_from_optioned(
                value.cluster_role_selectors,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: AggregationRuleAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.cluster_role_selectors,
            other.cluster_role_selectors,
        )?;
        Ok(())
    }
}
