#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// SuccessPolicyRule describes rule for declaring a Job as succeeded. Each rule must have at least one of the "succeededIndexes" or "succeededCount" specified.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SuccessPolicyRuleAc {
    /// succeededCount specifies the minimal required size of the actual set of the succeeded indexes for the Job. When succeededCount is used along with succeededIndexes, the check is constrained only to the set of indexes specified by succeededIndexes. For example, given that succeededIndexes is "1-4", succeededCount is "3", and completed indexes are "1", "3", and "5", the Job isn't declared as succeeded because only "1" and "3" indexes are considered in that rules. When this field is null, this doesn't default to any value and is never evaluated at any time. When specified it needs to be a positive integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded_count: Option<i32>,
    /// succeededIndexes specifies the set of indexes which need to be contained in the actual set of the succeeded indexes for the Job. The list of indexes must be within 0 to ".spec.completions-1" and must not contain duplicates. At least one element is required. The indexes are represented as intervals separated by commas. The intervals can be a decimal integer or a pair of decimal integers separated by a hyphen. The number are listed in represented by the first and last element of the series, separated by a hyphen. For example, if the completed indexes are 1, 3, 4, 5 and 7, they are represented as "1,3-5,7". When this field is null, this field doesn't default to any value and is never evaluated at any time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded_indexes: Option<std::string::String>,
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
            succeeded_count: self.succeeded_count,
            succeeded_indexes: self.succeeded_indexes,
        }
    }
    fn try_from_optioned(value: SuccessPolicyRuleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            succeeded_count: value.succeeded_count,
            succeeded_indexes: value.succeeded_indexes,
        })
    }
    fn merge(&mut self, other: SuccessPolicyRuleAc) -> Result<(), crate::Error> {
        self.succeeded_count = other.succeeded_count;
        self.succeeded_indexes = other.succeeded_indexes;
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
