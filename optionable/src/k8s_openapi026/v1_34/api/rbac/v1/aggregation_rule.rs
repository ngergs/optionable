#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AggregationRuleAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_role_selectors: <Option<
        std::vec::Vec<::k8s_openapi026::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::rbac::v1::AggregationRule {
    type Optioned = AggregationRuleAc;
}
#[automatically_derived]
impl crate::Optionable for AggregationRuleAc {
    type Optioned = AggregationRuleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::rbac::v1::AggregationRule {
    fn into_optioned(self) -> AggregationRuleAc {
        AggregationRuleAc {
            cluster_role_selectors: crate::OptionableConvert::into_optioned(
                self.cluster_role_selectors,
            ),
        }
    }
    fn try_from_optioned(value: AggregationRuleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            cluster_role_selectors: crate::OptionableConvert::try_from_optioned(
                value.cluster_role_selectors,
            )?,
        })
    }
    fn merge(&mut self, other: AggregationRuleAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.cluster_role_selectors,
            other.cluster_role_selectors,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::rbac::v1::AggregationRule>
for AggregationRuleAc {
    fn from_optionable(value: k8s_openapi026::api::rbac::v1::AggregationRule) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::rbac::v1::AggregationRule, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::rbac::v1::AggregationRule,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
