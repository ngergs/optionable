#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SubjectRulesReviewStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_error: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_resource_rules: Option<
        <std::vec::Vec<
            ::k8s_openapi026::api::authorization::v1::NonResourceRule,
        > as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_rules: Option<
        <std::vec::Vec<
            ::k8s_openapi026::api::authorization::v1::ResourceRule,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::api::authorization::v1::SubjectRulesReviewStatus {
    type Optioned = SubjectRulesReviewStatusAc;
}
#[automatically_derived]
impl crate::Optionable for SubjectRulesReviewStatusAc {
    type Optioned = SubjectRulesReviewStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::authorization::v1::SubjectRulesReviewStatus {
    fn into_optioned(self) -> SubjectRulesReviewStatusAc {
        SubjectRulesReviewStatusAc {
            evaluation_error: crate::OptionableConvert::into_optioned(
                self.evaluation_error,
            ),
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
            evaluation_error: crate::OptionableConvert::try_from_optioned(
                value.evaluation_error,
            )?,
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
        crate::OptionableConvert::merge(
            &mut self.evaluation_error,
            other.evaluation_error,
        )?;
        if let Some(other_value) = other.incomplete {
            self.incomplete = other_value;
        }
        if let Some(other_value) = other.non_resource_rules {
            crate::OptionableConvert::merge(&mut self.non_resource_rules, other_value)?;
        }
        if let Some(other_value) = other.resource_rules {
            crate::OptionableConvert::merge(&mut self.resource_rules, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::api::authorization::v1::SubjectRulesReviewStatus,
> for SubjectRulesReviewStatusAc {
    fn from_optionable(
        value: k8s_openapi026::api::authorization::v1::SubjectRulesReviewStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::authorization::v1::SubjectRulesReviewStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::authorization::v1::SubjectRulesReviewStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
