pub struct SuccessPolicyRuleOpt {
    pub succeeded_count: <Option<i32> as crate::Optionable>::Optioned,
    pub succeeded_indexes: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::batch::v1::SuccessPolicyRule {
    type Optioned = SuccessPolicyRuleOpt;
}
#[automatically_derived]
impl crate::Optionable for SuccessPolicyRuleOpt {
    type Optioned = SuccessPolicyRuleOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::batch::v1::SuccessPolicyRule {
    fn into_optioned(self) -> SuccessPolicyRuleOpt {
        SuccessPolicyRuleOpt {
            succeeded_count: crate::OptionableConvert::into_optioned(
                self.succeeded_count,
            ),
            succeeded_indexes: crate::OptionableConvert::into_optioned(
                self.succeeded_indexes,
            ),
        }
    }
    fn try_from_optioned(
        value: SuccessPolicyRuleOpt,
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
        other: SuccessPolicyRuleOpt,
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
