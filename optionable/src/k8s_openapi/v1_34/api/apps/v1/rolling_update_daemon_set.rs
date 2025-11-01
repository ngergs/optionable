pub struct RollingUpdateDaemonSetAc {
    pub max_surge: <Option<
        ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
    > as crate::Optionable>::Optioned,
    pub max_unavailable: <Option<
        ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::apps::v1::RollingUpdateDaemonSet {
    type Optioned = RollingUpdateDaemonSetAc;
}
#[automatically_derived]
impl crate::Optionable for RollingUpdateDaemonSetAc {
    type Optioned = RollingUpdateDaemonSetAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::apps::v1::RollingUpdateDaemonSet {
    fn into_optioned(self) -> RollingUpdateDaemonSetAc {
        RollingUpdateDaemonSetAc {
            max_surge: crate::OptionableConvert::into_optioned(self.max_surge),
            max_unavailable: crate::OptionableConvert::into_optioned(
                self.max_unavailable,
            ),
        }
    }
    fn try_from_optioned(
        value: RollingUpdateDaemonSetAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            max_surge: crate::OptionableConvert::try_from_optioned(value.max_surge)?,
            max_unavailable: crate::OptionableConvert::try_from_optioned(
                value.max_unavailable,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: RollingUpdateDaemonSetAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.max_surge, other.max_surge)?;
        crate::OptionableConvert::merge(
            &mut self.max_unavailable,
            other.max_unavailable,
        )?;
        Ok(())
    }
}
