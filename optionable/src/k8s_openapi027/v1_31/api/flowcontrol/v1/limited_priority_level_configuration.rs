#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// LimitedPriorityLevelConfiguration specifies how to handle requests that are subject to limits. It addresses two issues:
///   - How are requests for this priority level limited?
///   - What should be done with requests that exceed the limit?
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LimitedPriorityLevelConfigurationAc {
    /// `borrowingLimitPercent`, if present, configures a limit on how many seats this priority level can borrow from other priority levels. The limit is known as this level's BorrowingConcurrencyLimit (BorrowingCL) and is a limit on the total number of seats that this level may borrow at any one time. This field holds the ratio of that limit to the level's nominal concurrency limit. When this field is non-nil, it must hold a non-negative integer and the limit is calculated as follows.
    ///
    /// BorrowingCL(i) = round( NominalCL(i) * borrowingLimitPercent(i)/100.0 )
    ///
    /// The value of this field can be more than 100, implying that this priority level can borrow a number of seats that is greater than its own nominal concurrency limit (NominalCL). When this field is left `nil`, the limit is effectively infinite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrowing_limit_percent: Option<i32>,
    /// `lendablePercent` prescribes the fraction of the level's NominalCL that can be borrowed by other priority levels. The value of this field must be between 0 and 100, inclusive, and it defaults to 0. The number of seats that other levels can borrow from this level, known as this level's LendableConcurrencyLimit (LendableCL), is defined as follows.
    ///
    /// LendableCL(i) = round( NominalCL(i) * lendablePercent(i)/100.0 )
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lendable_percent: Option<i32>,
    /// `limitResponse` indicates what to do with requests that can not be executed right now
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_response: Option<
        <::k8s_openapi027::api::flowcontrol::v1::LimitResponse as crate::Optionable>::Optioned,
    >,
    /// `nominalConcurrencyShares` (NCS) contributes to the computation of the NominalConcurrencyLimit (NominalCL) of this level. This is the number of execution seats available at this priority level. This is used both for requests dispatched from this priority level as well as requests dispatched from other priority levels borrowing seats from this level. The server's concurrency limit (ServerCL) is divided among the Limited priority levels in proportion to their NCS values:
    ///
    /// NominalCL(i)  = ceil( ServerCL * NCS(i) / sum_ncs ) sum_ncs = sum\[priority level k\] NCS(k)
    ///
    /// Bigger numbers mean a larger nominal concurrency limit, at the expense of every other priority level.
    ///
    /// If not specified, this field defaults to a value of 30.
    ///
    /// Setting this field to zero supports the construction of a "jail" for this priority level that is used to hold some request(s)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nominal_concurrency_shares: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::flowcontrol::v1::LimitedPriorityLevelConfiguration {
    type Optioned = LimitedPriorityLevelConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for LimitedPriorityLevelConfigurationAc {
    type Optioned = LimitedPriorityLevelConfigurationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::flowcontrol::v1::LimitedPriorityLevelConfiguration {
    fn into_optioned(self) -> LimitedPriorityLevelConfigurationAc {
        LimitedPriorityLevelConfigurationAc {
            borrowing_limit_percent: self.borrowing_limit_percent,
            lendable_percent: self.lendable_percent,
            limit_response: crate::OptionableConvert::into_optioned(self.limit_response),
            nominal_concurrency_shares: self.nominal_concurrency_shares,
        }
    }
    fn try_from_optioned(
        value: LimitedPriorityLevelConfigurationAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            borrowing_limit_percent: value.borrowing_limit_percent,
            lendable_percent: value.lendable_percent,
            limit_response: crate::OptionableConvert::try_from_optioned(
                value.limit_response,
            )?,
            nominal_concurrency_shares: value.nominal_concurrency_shares,
        })
    }
    fn merge(
        &mut self,
        other: LimitedPriorityLevelConfigurationAc,
    ) -> Result<(), crate::Error> {
        self.borrowing_limit_percent = other.borrowing_limit_percent;
        self.lendable_percent = other.lendable_percent;
        crate::OptionableConvert::merge(&mut self.limit_response, other.limit_response)?;
        self.nominal_concurrency_shares = other.nominal_concurrency_shares;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::flowcontrol::v1::LimitedPriorityLevelConfiguration,
> for LimitedPriorityLevelConfigurationAc {
    fn from_optionable(
        value: k8s_openapi027::api::flowcontrol::v1::LimitedPriorityLevelConfiguration,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::flowcontrol::v1::LimitedPriorityLevelConfiguration,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::flowcontrol::v1::LimitedPriorityLevelConfiguration,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
