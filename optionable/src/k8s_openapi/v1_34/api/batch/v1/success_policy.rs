pub struct SuccessPolicyOpt {
    pub rules: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::batch::v1::SuccessPolicyRule,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::batch::v1::SuccessPolicy {
    type Optioned = SuccessPolicyOpt;
}
#[automatically_derived]
impl crate::Optionable for SuccessPolicyOpt {
    type Optioned = SuccessPolicyOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::batch::v1::SuccessPolicy {
    fn into_optioned(self) -> SuccessPolicyOpt {
        SuccessPolicyOpt {
            rules: Some(
                <std::vec::Vec<
                    ::k8s_openapi::api::batch::v1::SuccessPolicyRule,
                > as crate::OptionableConvert>::into_optioned(self.rules),
            ),
        }
    }
    fn try_from_optioned(
        value: SuccessPolicyOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            rules: <std::vec::Vec<
                ::k8s_openapi::api::batch::v1::SuccessPolicyRule,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .rules
                    .ok_or(crate::optionable::Error {
                        missing_field: "rules",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: SuccessPolicyOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.rules {
            <std::vec::Vec<
                ::k8s_openapi::api::batch::v1::SuccessPolicyRule,
            > as crate::OptionableConvert>::merge(&mut self.rules, other_value)?;
        }
        Ok(())
    }
}
