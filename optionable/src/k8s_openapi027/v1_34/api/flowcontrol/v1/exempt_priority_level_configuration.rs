#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ExemptPriorityLevelConfiguration describes the configurable aspects of the handling of exempt requests. In the mandatory exempt configuration object the values in the fields here can be modified by authorized users, unlike the rest of the `spec`.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExemptPriorityLevelConfigurationAc {
    /// `lendablePercent` prescribes the fraction of the level's NominalCL that can be borrowed by other priority levels.  This value of this field must be between 0 and 100, inclusive, and it defaults to 0. The number of seats that other levels can borrow from this level, known as this level's LendableConcurrencyLimit (LendableCL), is defined as follows.
    ///
    /// LendableCL(i) = round( NominalCL(i) * lendablePercent(i)/100.0 )
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lendable_percent: Option<i32>,
    /// `nominalConcurrencyShares` (NCS) contributes to the computation of the NominalConcurrencyLimit (NominalCL) of this level. This is the number of execution seats nominally reserved for this priority level. This DOES NOT limit the dispatching from this priority level but affects the other priority levels through the borrowing mechanism. The server's concurrency limit (ServerCL) is divided among all the priority levels in proportion to their NCS values:
    ///
    /// NominalCL(i)  = ceil( ServerCL * NCS(i) / sum_ncs ) sum_ncs = sum\[priority level k\] NCS(k)
    ///
    /// Bigger numbers mean a larger nominal concurrency limit, at the expense of every other priority level. This field has a default value of zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nominal_concurrency_shares: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::flowcontrol::v1::ExemptPriorityLevelConfiguration {
    type Optioned = ExemptPriorityLevelConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for ExemptPriorityLevelConfigurationAc {
    type Optioned = ExemptPriorityLevelConfigurationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::flowcontrol::v1::ExemptPriorityLevelConfiguration {
    fn into_optioned(self) -> ExemptPriorityLevelConfigurationAc {
        ExemptPriorityLevelConfigurationAc {
            lendable_percent: self.lendable_percent,
            nominal_concurrency_shares: self.nominal_concurrency_shares,
        }
    }
    fn try_from_optioned(
        value: ExemptPriorityLevelConfigurationAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            lendable_percent: value.lendable_percent,
            nominal_concurrency_shares: value.nominal_concurrency_shares,
        })
    }
    fn merge(
        &mut self,
        other: ExemptPriorityLevelConfigurationAc,
    ) -> Result<(), crate::Error> {
        if other.lendable_percent.is_some() {
            self.lendable_percent = other.lendable_percent;
        }
        if other.nominal_concurrency_shares.is_some() {
            self.nominal_concurrency_shares = other.nominal_concurrency_shares;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::flowcontrol::v1::ExemptPriorityLevelConfiguration,
> for ExemptPriorityLevelConfigurationAc {
    fn from_optionable(
        value: k8s_openapi027::api::flowcontrol::v1::ExemptPriorityLevelConfiguration,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::flowcontrol::v1::ExemptPriorityLevelConfiguration,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::flowcontrol::v1::ExemptPriorityLevelConfiguration,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
