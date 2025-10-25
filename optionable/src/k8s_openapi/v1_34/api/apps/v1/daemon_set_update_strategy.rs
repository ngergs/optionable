pub struct DaemonSetUpdateStrategyOpt {
    pub rolling_update: <Option<
        ::k8s_openapi::api::apps::v1::RollingUpdateDaemonSet,
    > as crate::Optionable>::Optioned,
    pub type_: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::apps::v1::daemon_set_update_strategy::DaemonSetUpdateStrategy {
    type Optioned = DaemonSetUpdateStrategyOpt;
}
#[automatically_derived]
impl crate::Optionable for DaemonSetUpdateStrategyOpt {
    type Optioned = DaemonSetUpdateStrategyOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::apps::v1::daemon_set_update_strategy::DaemonSetUpdateStrategy {
    fn into_optioned(self) -> DaemonSetUpdateStrategyOpt {
        DaemonSetUpdateStrategyOpt {
            rolling_update: <Option<
                ::k8s_openapi::api::apps::v1::RollingUpdateDaemonSet,
            > as crate::OptionableConvert>::into_optioned(self.rolling_update),
            type_: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.type_),
        }
    }
    fn try_from_optioned(
        value: DaemonSetUpdateStrategyOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            rolling_update: <Option<
                ::k8s_openapi::api::apps::v1::RollingUpdateDaemonSet,
            > as crate::OptionableConvert>::try_from_optioned(value.rolling_update)?,
            type_: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.type_)?,
        })
    }
    fn merge(
        &mut self,
        other: DaemonSetUpdateStrategyOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::apps::v1::RollingUpdateDaemonSet,
        > as crate::OptionableConvert>::merge(
            &mut self.rolling_update,
            other.rolling_update,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.type_, other.type_)?;
        Ok(())
    }
}
