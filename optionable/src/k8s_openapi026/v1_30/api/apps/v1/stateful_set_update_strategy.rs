#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StatefulSetUpdateStrategyAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_update: <Option<
        ::k8s_openapi026::api::apps::v1::RollingUpdateStatefulSetStrategy,
    > as crate::Optionable>::Optioned,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::apps::v1::StatefulSetUpdateStrategy {
    type Optioned = StatefulSetUpdateStrategyAc;
}
#[automatically_derived]
impl crate::Optionable for StatefulSetUpdateStrategyAc {
    type Optioned = StatefulSetUpdateStrategyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::apps::v1::StatefulSetUpdateStrategy {
    fn into_optioned(self) -> StatefulSetUpdateStrategyAc {
        StatefulSetUpdateStrategyAc {
            rolling_update: crate::OptionableConvert::into_optioned(self.rolling_update),
            type_: crate::OptionableConvert::into_optioned(self.type_),
        }
    }
    fn try_from_optioned(
        value: StatefulSetUpdateStrategyAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            rolling_update: crate::OptionableConvert::try_from_optioned(
                value.rolling_update,
            )?,
            type_: crate::OptionableConvert::try_from_optioned(value.type_)?,
        })
    }
    fn merge(&mut self, other: StatefulSetUpdateStrategyAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.rolling_update, other.rolling_update)?;
        crate::OptionableConvert::merge(&mut self.type_, other.type_)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::apps::v1::StatefulSetUpdateStrategy>
for StatefulSetUpdateStrategyAc {
    fn from_optionable(
        value: k8s_openapi026::api::apps::v1::StatefulSetUpdateStrategy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::apps::v1::StatefulSetUpdateStrategy, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::apps::v1::StatefulSetUpdateStrategy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
