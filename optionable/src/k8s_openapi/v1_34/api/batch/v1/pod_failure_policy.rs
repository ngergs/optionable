pub struct PodFailurePolicyOpt {
    pub rules: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::batch::v1::PodFailurePolicyRule,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::batch::v1::PodFailurePolicy {
    type Optioned = PodFailurePolicyOpt;
}
#[automatically_derived]
impl crate::Optionable for PodFailurePolicyOpt {
    type Optioned = PodFailurePolicyOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::batch::v1::PodFailurePolicy {
    fn into_optioned(self) -> PodFailurePolicyOpt {
        PodFailurePolicyOpt {
            rules: Some(
                <std::vec::Vec<
                    ::k8s_openapi::api::batch::v1::PodFailurePolicyRule,
                > as crate::OptionableConvert>::into_optioned(self.rules),
            ),
        }
    }
    fn try_from_optioned(
        value: PodFailurePolicyOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            rules: <std::vec::Vec<
                ::k8s_openapi::api::batch::v1::PodFailurePolicyRule,
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
        other: PodFailurePolicyOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.rules {
            <std::vec::Vec<
                ::k8s_openapi::api::batch::v1::PodFailurePolicyRule,
            > as crate::OptionableConvert>::merge(&mut self.rules, other_value)?;
        }
        Ok(())
    }
}
