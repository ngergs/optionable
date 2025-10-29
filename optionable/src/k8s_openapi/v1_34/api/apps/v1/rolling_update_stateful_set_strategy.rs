pub struct RollingUpdateStatefulSetStrategyOpt {
    pub max_unavailable: <Option<
        ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
    > as crate::Optionable>::Optioned,
    pub partition: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::apps::v1::RollingUpdateStatefulSetStrategy {
    type Optioned = RollingUpdateStatefulSetStrategyOpt;
}
#[automatically_derived]
impl crate::Optionable for RollingUpdateStatefulSetStrategyOpt {
    type Optioned = RollingUpdateStatefulSetStrategyOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::apps::v1::RollingUpdateStatefulSetStrategy {
    fn into_optioned(self) -> RollingUpdateStatefulSetStrategyOpt {
        RollingUpdateStatefulSetStrategyOpt {
            max_unavailable: crate::OptionableConvert::into_optioned(
                self.max_unavailable,
            ),
            partition: crate::OptionableConvert::into_optioned(self.partition),
        }
    }
    fn try_from_optioned(
        value: RollingUpdateStatefulSetStrategyOpt,
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
        other: RollingUpdateStatefulSetStrategyOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.max_unavailable,
            other.max_unavailable,
        )?;
        crate::OptionableConvert::merge(&mut self.partition, other.partition)?;
        Ok(())
    }
}
