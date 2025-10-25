pub struct RollingUpdateDaemonSetOpt {
    pub max_surge: <Option<
        ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
    > as crate::Optionable>::Optioned,
    pub max_unavailable: <Option<
        ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::apps::v1::rolling_update_daemon_set::RollingUpdateDaemonSet {
    type Optioned = RollingUpdateDaemonSetOpt;
}
#[automatically_derived]
impl crate::Optionable for RollingUpdateDaemonSetOpt {
    type Optioned = RollingUpdateDaemonSetOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::apps::v1::rolling_update_daemon_set::RollingUpdateDaemonSet {
    fn into_optioned(self) -> RollingUpdateDaemonSetOpt {
        RollingUpdateDaemonSetOpt {
            max_surge: <Option<
                ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
            > as crate::OptionableConvert>::into_optioned(self.max_surge),
            max_unavailable: <Option<
                ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
            > as crate::OptionableConvert>::into_optioned(self.max_unavailable),
        }
    }
    fn try_from_optioned(
        value: RollingUpdateDaemonSetOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            max_surge: <Option<
                ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
            > as crate::OptionableConvert>::try_from_optioned(value.max_surge)?,
            max_unavailable: <Option<
                ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
            > as crate::OptionableConvert>::try_from_optioned(value.max_unavailable)?,
        })
    }
    fn merge(
        &mut self,
        other: RollingUpdateDaemonSetOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
        > as crate::OptionableConvert>::merge(&mut self.max_surge, other.max_surge)?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
        > as crate::OptionableConvert>::merge(
            &mut self.max_unavailable,
            other.max_unavailable,
        )?;
        Ok(())
    }
}
