pub struct SelfSubjectRulesReviewSpecOpt {
    pub namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::authorization::v1::SelfSubjectRulesReviewSpec {
    type Optioned = SelfSubjectRulesReviewSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for SelfSubjectRulesReviewSpecOpt {
    type Optioned = SelfSubjectRulesReviewSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::SelfSubjectRulesReviewSpec {
    fn into_optioned(self) -> SelfSubjectRulesReviewSpecOpt {
        SelfSubjectRulesReviewSpecOpt {
            namespace: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.namespace),
        }
    }
    fn try_from_optioned(
        value: SelfSubjectRulesReviewSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            namespace: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.namespace)?,
        })
    }
    fn merge(
        &mut self,
        other: SelfSubjectRulesReviewSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.namespace, other.namespace)?;
        Ok(())
    }
}
