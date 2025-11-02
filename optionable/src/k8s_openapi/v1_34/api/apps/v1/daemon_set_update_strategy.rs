#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DaemonSetUpdateStrategyAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_update: <Option<
        ::k8s_openapi::api::apps::v1::RollingUpdateDaemonSet,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::apps::v1::DaemonSetUpdateStrategy {
    type Optioned = DaemonSetUpdateStrategyAc;
}
#[automatically_derived]
impl crate::Optionable for DaemonSetUpdateStrategyAc {
    type Optioned = DaemonSetUpdateStrategyAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::apps::v1::DaemonSetUpdateStrategy {
    fn into_optioned(self) -> DaemonSetUpdateStrategyAc {
        DaemonSetUpdateStrategyAc {
            rolling_update: crate::OptionableConvert::into_optioned(self.rolling_update),
            type_: crate::OptionableConvert::into_optioned(self.type_),
        }
    }
    fn try_from_optioned(
        value: DaemonSetUpdateStrategyAc,
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
        other: DaemonSetUpdateStrategyAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.rolling_update, other.rolling_update)?;
        crate::OptionableConvert::merge(&mut self.type_, other.type_)?;
        Ok(())
    }
}
