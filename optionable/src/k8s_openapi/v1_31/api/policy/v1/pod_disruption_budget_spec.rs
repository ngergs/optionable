#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct PodDisruptionBudgetSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable: <Option<
        ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_available: <Option<
        ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_pod_eviction_policy: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::policy::v1::PodDisruptionBudgetSpec {
    type Optioned = PodDisruptionBudgetSpecAc;
}
#[automatically_derived]
impl crate::Optionable for PodDisruptionBudgetSpecAc {
    type Optioned = PodDisruptionBudgetSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::policy::v1::PodDisruptionBudgetSpec {
    fn into_optioned(self) -> PodDisruptionBudgetSpecAc {
        PodDisruptionBudgetSpecAc {
            max_unavailable: crate::OptionableConvert::into_optioned(
                self.max_unavailable,
            ),
            min_available: crate::OptionableConvert::into_optioned(self.min_available),
            selector: crate::OptionableConvert::into_optioned(self.selector),
            unhealthy_pod_eviction_policy: crate::OptionableConvert::into_optioned(
                self.unhealthy_pod_eviction_policy,
            ),
        }
    }
    fn try_from_optioned(
        value: PodDisruptionBudgetSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            max_unavailable: crate::OptionableConvert::try_from_optioned(
                value.max_unavailable,
            )?,
            min_available: crate::OptionableConvert::try_from_optioned(
                value.min_available,
            )?,
            selector: crate::OptionableConvert::try_from_optioned(value.selector)?,
            unhealthy_pod_eviction_policy: crate::OptionableConvert::try_from_optioned(
                value.unhealthy_pod_eviction_policy,
            )?,
        })
    }
    fn merge(&mut self, other: PodDisruptionBudgetSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.max_unavailable,
            other.max_unavailable,
        )?;
        crate::OptionableConvert::merge(&mut self.min_available, other.min_available)?;
        crate::OptionableConvert::merge(&mut self.selector, other.selector)?;
        crate::OptionableConvert::merge(
            &mut self.unhealthy_pod_eviction_policy,
            other.unhealthy_pod_eviction_policy,
        )?;
        Ok(())
    }
}
