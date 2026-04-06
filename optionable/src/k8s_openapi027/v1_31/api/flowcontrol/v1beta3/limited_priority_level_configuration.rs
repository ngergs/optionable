#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LimitedPriorityLevelConfigurationAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrowing_limit_percent: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lendable_percent: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_response: <Option<
        ::k8s_openapi027::api::flowcontrol::v1beta3::LimitResponse,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nominal_concurrency_shares: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::flowcontrol::v1beta3::LimitedPriorityLevelConfiguration {
    type Optioned = LimitedPriorityLevelConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for LimitedPriorityLevelConfigurationAc {
    type Optioned = LimitedPriorityLevelConfigurationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::flowcontrol::v1beta3::LimitedPriorityLevelConfiguration {
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
    k8s_openapi027::api::flowcontrol::v1beta3::LimitedPriorityLevelConfiguration,
> for LimitedPriorityLevelConfigurationAc {
    fn from_optionable(
        value: k8s_openapi027::api::flowcontrol::v1beta3::LimitedPriorityLevelConfiguration,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::flowcontrol::v1beta3::LimitedPriorityLevelConfiguration,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::flowcontrol::v1beta3::LimitedPriorityLevelConfiguration,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
