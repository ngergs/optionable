#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessPolicyRuleAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded_count: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded_indexes: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::batch::v1::SuccessPolicyRule {
    type Optioned = SuccessPolicyRuleAc;
}
#[automatically_derived]
impl crate::Optionable for SuccessPolicyRuleAc {
    type Optioned = SuccessPolicyRuleAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::batch::v1::SuccessPolicyRule {
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
    fn try_from_optioned(
        value: SuccessPolicyRuleAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            succeeded_count: crate::OptionableConvert::try_from_optioned(
                value.succeeded_count,
            )?,
            succeeded_indexes: crate::OptionableConvert::try_from_optioned(
                value.succeeded_indexes,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: SuccessPolicyRuleAc,
    ) -> Result<(), crate::optionable::Error> {
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
