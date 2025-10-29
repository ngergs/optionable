pub struct StatefulSetUpdateStrategyOpt {
    pub rolling_update: <Option<
        ::k8s_openapi::api::apps::v1::RollingUpdateStatefulSetStrategy,
    > as crate::Optionable>::Optioned,
    pub type_: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::apps::v1::StatefulSetUpdateStrategy {
    type Optioned = StatefulSetUpdateStrategyOpt;
}
#[automatically_derived]
impl crate::Optionable for StatefulSetUpdateStrategyOpt {
    type Optioned = StatefulSetUpdateStrategyOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::apps::v1::StatefulSetUpdateStrategy {
    fn into_optioned(self) -> StatefulSetUpdateStrategyOpt {
        StatefulSetUpdateStrategyOpt {
            rolling_update: crate::OptionableConvert::into_optioned(self.rolling_update),
            type_: crate::OptionableConvert::into_optioned(self.type_),
        }
    }
    fn try_from_optioned(
        value: StatefulSetUpdateStrategyOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            rolling_update: crate::OptionableConvert::try_from_optioned(
                value.rolling_update,
            )?,
            type_: crate::OptionableConvert::try_from_optioned(value.type_)?,
        })
    }
    fn merge(
        &mut self,
        other: StatefulSetUpdateStrategyOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.rolling_update, other.rolling_update)?;
        crate::OptionableConvert::merge(&mut self.type_, other.type_)?;
        Ok(())
    }
}
