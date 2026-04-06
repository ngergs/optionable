#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExemptPriorityLevelConfigurationAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lendable_percent: Option<i32>,
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
        self.lendable_percent = other.lendable_percent;
        self.nominal_concurrency_shares = other.nominal_concurrency_shares;
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
