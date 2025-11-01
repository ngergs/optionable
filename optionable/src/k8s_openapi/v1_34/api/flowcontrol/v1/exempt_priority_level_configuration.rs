pub struct ExemptPriorityLevelConfigurationAc {
    pub lendable_percent: <Option<i32> as crate::Optionable>::Optioned,
    pub nominal_concurrency_shares: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::flowcontrol::v1::ExemptPriorityLevelConfiguration {
    type Optioned = ExemptPriorityLevelConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for ExemptPriorityLevelConfigurationAc {
    type Optioned = ExemptPriorityLevelConfigurationAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::ExemptPriorityLevelConfiguration {
    fn into_optioned(self) -> ExemptPriorityLevelConfigurationAc {
        ExemptPriorityLevelConfigurationAc {
            lendable_percent: crate::OptionableConvert::into_optioned(
                self.lendable_percent,
            ),
            nominal_concurrency_shares: crate::OptionableConvert::into_optioned(
                self.nominal_concurrency_shares,
            ),
        }
    }
    fn try_from_optioned(
        value: ExemptPriorityLevelConfigurationAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            lendable_percent: crate::OptionableConvert::try_from_optioned(
                value.lendable_percent,
            )?,
            nominal_concurrency_shares: crate::OptionableConvert::try_from_optioned(
                value.nominal_concurrency_shares,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ExemptPriorityLevelConfigurationAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.lendable_percent,
            other.lendable_percent,
        )?;
        crate::OptionableConvert::merge(
            &mut self.nominal_concurrency_shares,
            other.nominal_concurrency_shares,
        )?;
        Ok(())
    }
}
