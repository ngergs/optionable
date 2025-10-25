pub struct LimitedPriorityLevelConfigurationOpt {
    pub borrowing_limit_percent: <Option<i32> as crate::Optionable>::Optioned,
    pub lendable_percent: <Option<i32> as crate::Optionable>::Optioned,
    pub limit_response: <Option<
        ::k8s_openapi::api::flowcontrol::v1::LimitResponse,
    > as crate::Optionable>::Optioned,
    pub nominal_concurrency_shares: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::flowcontrol::v1::limited_priority_level_configuration::LimitedPriorityLevelConfiguration {
    type Optioned = LimitedPriorityLevelConfigurationOpt;
}
#[automatically_derived]
impl crate::Optionable for LimitedPriorityLevelConfigurationOpt {
    type Optioned = LimitedPriorityLevelConfigurationOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::limited_priority_level_configuration::LimitedPriorityLevelConfiguration {
    fn into_optioned(self) -> LimitedPriorityLevelConfigurationOpt {
        LimitedPriorityLevelConfigurationOpt {
            borrowing_limit_percent: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.borrowing_limit_percent),
            lendable_percent: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.lendable_percent),
            limit_response: <Option<
                ::k8s_openapi::api::flowcontrol::v1::LimitResponse,
            > as crate::OptionableConvert>::into_optioned(self.limit_response),
            nominal_concurrency_shares: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(
                self.nominal_concurrency_shares,
            ),
        }
    }
    fn try_from_optioned(
        value: LimitedPriorityLevelConfigurationOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            borrowing_limit_percent: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(
                value.borrowing_limit_percent,
            )?,
            lendable_percent: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.lendable_percent)?,
            limit_response: <Option<
                ::k8s_openapi::api::flowcontrol::v1::LimitResponse,
            > as crate::OptionableConvert>::try_from_optioned(value.limit_response)?,
            nominal_concurrency_shares: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(
                value.nominal_concurrency_shares,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: LimitedPriorityLevelConfigurationOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.borrowing_limit_percent,
            other.borrowing_limit_percent,
        )?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.lendable_percent,
            other.lendable_percent,
        )?;
        <Option<
            ::k8s_openapi::api::flowcontrol::v1::LimitResponse,
        > as crate::OptionableConvert>::merge(
            &mut self.limit_response,
            other.limit_response,
        )?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.nominal_concurrency_shares,
            other.nominal_concurrency_shares,
        )?;
        Ok(())
    }
}
