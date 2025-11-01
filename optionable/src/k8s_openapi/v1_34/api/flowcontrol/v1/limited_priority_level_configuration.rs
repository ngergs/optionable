pub struct LimitedPriorityLevelConfigurationAc {
    pub borrowing_limit_percent: <Option<i32> as crate::Optionable>::Optioned,
    pub lendable_percent: <Option<i32> as crate::Optionable>::Optioned,
    pub limit_response: <Option<
        ::k8s_openapi::api::flowcontrol::v1::LimitResponse,
    > as crate::Optionable>::Optioned,
    pub nominal_concurrency_shares: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::flowcontrol::v1::LimitedPriorityLevelConfiguration {
    type Optioned = LimitedPriorityLevelConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for LimitedPriorityLevelConfigurationAc {
    type Optioned = LimitedPriorityLevelConfigurationAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::LimitedPriorityLevelConfiguration {
    fn into_optioned(self) -> LimitedPriorityLevelConfigurationAc {
        LimitedPriorityLevelConfigurationAc {
            borrowing_limit_percent: crate::OptionableConvert::into_optioned(
                self.borrowing_limit_percent,
            ),
            lendable_percent: crate::OptionableConvert::into_optioned(
                self.lendable_percent,
            ),
            limit_response: crate::OptionableConvert::into_optioned(self.limit_response),
            nominal_concurrency_shares: crate::OptionableConvert::into_optioned(
                self.nominal_concurrency_shares,
            ),
        }
    }
    fn try_from_optioned(
        value: LimitedPriorityLevelConfigurationAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            borrowing_limit_percent: crate::OptionableConvert::try_from_optioned(
                value.borrowing_limit_percent,
            )?,
            lendable_percent: crate::OptionableConvert::try_from_optioned(
                value.lendable_percent,
            )?,
            limit_response: crate::OptionableConvert::try_from_optioned(
                value.limit_response,
            )?,
            nominal_concurrency_shares: crate::OptionableConvert::try_from_optioned(
                value.nominal_concurrency_shares,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: LimitedPriorityLevelConfigurationAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.borrowing_limit_percent,
            other.borrowing_limit_percent,
        )?;
        crate::OptionableConvert::merge(
            &mut self.lendable_percent,
            other.lendable_percent,
        )?;
        crate::OptionableConvert::merge(&mut self.limit_response, other.limit_response)?;
        crate::OptionableConvert::merge(
            &mut self.nominal_concurrency_shares,
            other.nominal_concurrency_shares,
        )?;
        Ok(())
    }
}
