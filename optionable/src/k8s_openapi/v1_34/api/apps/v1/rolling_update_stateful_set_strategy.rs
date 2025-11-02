#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RollingUpdateStatefulSetStrategyAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable: <Option<
        ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::apps::v1::RollingUpdateStatefulSetStrategy {
    type Optioned = RollingUpdateStatefulSetStrategyAc;
}
#[automatically_derived]
impl crate::Optionable for RollingUpdateStatefulSetStrategyAc {
    type Optioned = RollingUpdateStatefulSetStrategyAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::apps::v1::RollingUpdateStatefulSetStrategy {
    fn into_optioned(self) -> RollingUpdateStatefulSetStrategyAc {
        RollingUpdateStatefulSetStrategyAc {
            max_unavailable: crate::OptionableConvert::into_optioned(
                self.max_unavailable,
            ),
            partition: crate::OptionableConvert::into_optioned(self.partition),
        }
    }
    fn try_from_optioned(
        value: RollingUpdateStatefulSetStrategyAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            max_unavailable: crate::OptionableConvert::try_from_optioned(
                value.max_unavailable,
            )?,
            partition: crate::OptionableConvert::try_from_optioned(value.partition)?,
        })
    }
    fn merge(
        &mut self,
        other: RollingUpdateStatefulSetStrategyAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.max_unavailable,
            other.max_unavailable,
        )?;
        crate::OptionableConvert::merge(&mut self.partition, other.partition)?;
        Ok(())
    }
}
