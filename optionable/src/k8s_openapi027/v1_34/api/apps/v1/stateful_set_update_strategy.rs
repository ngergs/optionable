#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// StatefulSetUpdateStrategy indicates the strategy that the StatefulSet controller will use to perform updates. It includes any additional parameters necessary to perform the update for the indicated strategy.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StatefulSetUpdateStrategyAc {
    /// RollingUpdate is used to communicate parameters when Type is RollingUpdateStatefulSetStrategyType.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_update: Option<
        <::k8s_openapi027::api::apps::v1::RollingUpdateStatefulSetStrategy as crate::Optionable>::Optioned,
    >,
    /// Type indicates the type of the StatefulSetUpdateStrategy. Default is RollingUpdate.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::apps::v1::StatefulSetUpdateStrategy {
    type Optioned = StatefulSetUpdateStrategyAc;
}
#[automatically_derived]
impl crate::Optionable for StatefulSetUpdateStrategyAc {
    type Optioned = StatefulSetUpdateStrategyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::apps::v1::StatefulSetUpdateStrategy {
    fn into_optioned(self) -> StatefulSetUpdateStrategyAc {
        StatefulSetUpdateStrategyAc {
            rolling_update: crate::OptionableConvert::into_optioned(self.rolling_update),
            type_: self.type_,
        }
    }
    fn try_from_optioned(
        value: StatefulSetUpdateStrategyAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            rolling_update: crate::OptionableConvert::try_from_optioned(
                value.rolling_update,
            )?,
            type_: value.type_,
        })
    }
    fn merge(&mut self, other: StatefulSetUpdateStrategyAc) -> Result<(), crate::Error> {
        if self.rolling_update.is_none() {
            self.rolling_update = other.rolling_update;
        }
        if let Some(other_value) = other.rolling_update {
            crate::OptionableConvert::merge(&mut self.rolling_update, other_value)?;
        }
        if self.type_.is_none() {
            self.type_ = other.type_;
        }
        if let Some(other_value) = other.type_ {
            crate::OptionableConvert::merge(&mut self.type_, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::apps::v1::StatefulSetUpdateStrategy>
for StatefulSetUpdateStrategyAc {
    fn from_optionable(
        value: k8s_openapi027::api::apps::v1::StatefulSetUpdateStrategy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::apps::v1::StatefulSetUpdateStrategy, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apps::v1::StatefulSetUpdateStrategy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
