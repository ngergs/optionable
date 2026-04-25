#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// SubjectRulesReviewStatus contains the result of a rules check. This check can be incomplete depending on the set of authorizers the server is configured with and any errors experienced during evaluation. Because authorization rules are additive, if a rule appears in a list it's safe to assume the subject has that permission, even if that list is incomplete.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SubjectRulesReviewStatusAc {
    /// EvaluationError can appear in combination with Rules. It indicates an error occurred during rule evaluation, such as an authorizer that doesn't support rule evaluation, and that ResourceRules and/or NonResourceRules may be incomplete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_error: Option<std::string::String>,
    /// Incomplete is true when the rules returned by this call are incomplete. This is most commonly encountered when an authorizer, such as an external authorizer, doesn't support rules evaluation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete: Option<bool>,
    /// NonResourceRules is the list of actions the subject is allowed to perform on non-resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_resource_rules: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::authorization::v1::NonResourceRule as crate::Optionable>::Optioned,
        >,
    >,
    /// ResourceRules is the list of actions the subject is allowed to perform on resources. The list ordering isn't significant, may contain duplicates, and possibly be incomplete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_rules: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::authorization::v1::ResourceRule as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::authorization::v1::SubjectRulesReviewStatus {
    type Optioned = SubjectRulesReviewStatusAc;
}
#[automatically_derived]
impl crate::Optionable for SubjectRulesReviewStatusAc {
    type Optioned = SubjectRulesReviewStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::authorization::v1::SubjectRulesReviewStatus {
    fn into_optioned(self) -> SubjectRulesReviewStatusAc {
        SubjectRulesReviewStatusAc {
            evaluation_error: self.evaluation_error,
            incomplete: Some(self.incomplete),
            non_resource_rules: Some(
                crate::OptionableConvert::into_optioned(self.non_resource_rules),
            ),
            resource_rules: Some(
                crate::OptionableConvert::into_optioned(self.resource_rules),
            ),
        }
    }
    fn try_from_optioned(
        value: SubjectRulesReviewStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            evaluation_error: value.evaluation_error,
            incomplete: value
                .incomplete
                .ok_or(crate::Error {
                    missing_field: "incomplete",
                })?,
            non_resource_rules: crate::OptionableConvert::try_from_optioned(
                value
                    .non_resource_rules
                    .ok_or(crate::Error {
                        missing_field: "non_resource_rules",
                    })?,
            )?,
            resource_rules: crate::OptionableConvert::try_from_optioned(
                value
                    .resource_rules
                    .ok_or(crate::Error {
                        missing_field: "resource_rules",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: SubjectRulesReviewStatusAc) -> Result<(), crate::Error> {
        if self.evaluation_error.is_none() {
            self.evaluation_error = crate::OptionableConvert::try_from_optioned(
                other.evaluation_error,
            )?;
        } else if let Some(self_value) = self.evaluation_error.as_mut()
            && let Some(other_value) = other.evaluation_error
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.incomplete {
            self.incomplete = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.non_resource_rules {
            self.non_resource_rules = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if let Some(other_value) = other.resource_rules {
            self.resource_rules = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::authorization::v1::SubjectRulesReviewStatus,
> for SubjectRulesReviewStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::authorization::v1::SubjectRulesReviewStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::authorization::v1::SubjectRulesReviewStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::authorization::v1::SubjectRulesReviewStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for SubjectRulesReviewStatusAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.evaluation_error,
            other.evaluation_error,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.incomplete, other.incomplete);
        self.non_resource_rules = other.non_resource_rules;
        self.resource_rules = other.resource_rules;
    }
}
