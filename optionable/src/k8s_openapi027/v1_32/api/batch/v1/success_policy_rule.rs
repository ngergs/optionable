#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SuccessPolicyRuleAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded_count: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded_indexes: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::batch::v1::SuccessPolicyRule {
    type Optioned = SuccessPolicyRuleAc;
}
#[automatically_derived]
impl crate::Optionable for SuccessPolicyRuleAc {
    type Optioned = SuccessPolicyRuleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::batch::v1::SuccessPolicyRule {
    fn into_optioned(self) -> SuccessPolicyRuleAc {
        SuccessPolicyRuleAc {
            succeeded_count: crate::OptionableConvert::into_optioned(
                self.succeeded_count,
            ),
            succeeded_indexes: crate::OptionableConvert::into_optioned(
                self.succeeded_indexes,
            ),
        }
    }
    fn try_from_optioned(value: SuccessPolicyRuleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            succeeded_count: crate::OptionableConvert::try_from_optioned(
                value.succeeded_count,
            )?,
            succeeded_indexes: crate::OptionableConvert::try_from_optioned(
                value.succeeded_indexes,
            )?,
        })
    }
    fn merge(&mut self, other: SuccessPolicyRuleAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.succeeded_count,
            other.succeeded_count,
        )?;
        crate::OptionableConvert::merge(
            &mut self.succeeded_indexes,
            other.succeeded_indexes,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::batch::v1::SuccessPolicyRule>
for SuccessPolicyRuleAc {
    fn from_optionable(
        value: k8s_openapi027::api::batch::v1::SuccessPolicyRule,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::batch::v1::SuccessPolicyRule, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::batch::v1::SuccessPolicyRule,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
