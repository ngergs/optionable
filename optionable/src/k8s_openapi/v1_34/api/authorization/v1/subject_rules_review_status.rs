pub struct SubjectRulesReviewStatusOpt {
    pub evaluation_error: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub incomplete: Option<bool>,
    pub non_resource_rules: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::authorization::v1::NonResourceRule,
        > as crate::Optionable>::Optioned,
    >,
    pub resource_rules: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::authorization::v1::ResourceRule,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::authorization::v1::subject_rules_review_status::SubjectRulesReviewStatus {
    type Optioned = SubjectRulesReviewStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for SubjectRulesReviewStatusOpt {
    type Optioned = SubjectRulesReviewStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::subject_rules_review_status::SubjectRulesReviewStatus {
    fn into_optioned(self) -> SubjectRulesReviewStatusOpt {
        SubjectRulesReviewStatusOpt {
            evaluation_error: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.evaluation_error),
            incomplete: Some(self.incomplete),
            non_resource_rules: Some(
                <std::vec::Vec<
                    ::k8s_openapi::api::authorization::v1::NonResourceRule,
                > as crate::OptionableConvert>::into_optioned(self.non_resource_rules),
            ),
            resource_rules: Some(
                <std::vec::Vec<
                    ::k8s_openapi::api::authorization::v1::ResourceRule,
                > as crate::OptionableConvert>::into_optioned(self.resource_rules),
            ),
        }
    }
    fn try_from_optioned(
        value: SubjectRulesReviewStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            evaluation_error: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.evaluation_error)?,
            incomplete: value
                .incomplete
                .ok_or(crate::optionable::Error {
                    missing_field: "incomplete",
                })?,
            non_resource_rules: <std::vec::Vec<
                ::k8s_openapi::api::authorization::v1::NonResourceRule,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .non_resource_rules
                    .ok_or(crate::optionable::Error {
                        missing_field: "non_resource_rules",
                    })?,
            )?,
            resource_rules: <std::vec::Vec<
                ::k8s_openapi::api::authorization::v1::ResourceRule,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .resource_rules
                    .ok_or(crate::optionable::Error {
                        missing_field: "resource_rules",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: SubjectRulesReviewStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.evaluation_error,
            other.evaluation_error,
        )?;
        if let Some(other_value) = other.incomplete {
            self.incomplete = other_value;
        }
        if let Some(other_value) = other.non_resource_rules {
            <std::vec::Vec<
                ::k8s_openapi::api::authorization::v1::NonResourceRule,
            > as crate::OptionableConvert>::merge(
                &mut self.non_resource_rules,
                other_value,
            )?;
        }
        if let Some(other_value) = other.resource_rules {
            <std::vec::Vec<
                ::k8s_openapi::api::authorization::v1::ResourceRule,
            > as crate::OptionableConvert>::merge(
                &mut self.resource_rules,
                other_value,
            )?;
        }
        Ok(())
    }
}
