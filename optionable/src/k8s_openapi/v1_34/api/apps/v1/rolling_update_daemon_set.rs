#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RollingUpdateDaemonSetAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_surge: <Option<
        ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::apps::v1::RollingUpdateDaemonSet {
    fn into_optioned(self) -> RollingUpdateDaemonSetAc {
        RollingUpdateDaemonSetAc {
            max_surge: crate::OptionableConvert::into_optioned(self.max_surge),
            max_unavailable: crate::OptionableConvert::into_optioned(
                self.max_unavailable,
            ),
        }
    }
    fn try_from_optioned(value: RollingUpdateDaemonSetAc) -> Result<Self, crate::Error> {
        Ok(Self {
            max_surge: crate::OptionableConvert::try_from_optioned(value.max_surge)?,
            max_unavailable: crate::OptionableConvert::try_from_optioned(
                value.max_unavailable,
            )?,
        })
    }
    fn merge(&mut self, other: RollingUpdateDaemonSetAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.max_surge, other.max_surge)?;
        crate::OptionableConvert::merge(
            &mut self.max_unavailable,
            other.max_unavailable,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::apps::v1::RollingUpdateDaemonSet>
for RollingUpdateDaemonSetAc {
    fn from_optionable(
        value: ::k8s_openapi::api::apps::v1::RollingUpdateDaemonSet,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::apps::v1::RollingUpdateDaemonSet, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::apps::v1::RollingUpdateDaemonSet,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
