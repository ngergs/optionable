pub struct PodDisruptionBudgetSpecOpt {
    pub max_unavailable: <Option<
        ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
    > as crate::Optionable>::Optioned,
    pub min_available: <Option<
        ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
    > as crate::Optionable>::Optioned,
    pub selector: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    pub unhealthy_pod_eviction_policy: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::policy::v1::PodDisruptionBudgetSpec {
    type Optioned = PodDisruptionBudgetSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for PodDisruptionBudgetSpecOpt {
    type Optioned = PodDisruptionBudgetSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::policy::v1::PodDisruptionBudgetSpec {
    fn into_optioned(self) -> PodDisruptionBudgetSpecOpt {
        PodDisruptionBudgetSpecOpt {
            max_unavailable: <Option<
                ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
            > as crate::OptionableConvert>::into_optioned(self.max_unavailable),
            min_available: <Option<
                ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
            > as crate::OptionableConvert>::into_optioned(self.min_available),
            selector: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
            > as crate::OptionableConvert>::into_optioned(self.selector),
            unhealthy_pod_eviction_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(
                self.unhealthy_pod_eviction_policy,
            ),
        }
    }
    fn try_from_optioned(
        value: PodDisruptionBudgetSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            max_unavailable: <Option<
                ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
            > as crate::OptionableConvert>::try_from_optioned(value.max_unavailable)?,
            min_available: <Option<
                ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
            > as crate::OptionableConvert>::try_from_optioned(value.min_available)?,
            selector: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
            > as crate::OptionableConvert>::try_from_optioned(value.selector)?,
            unhealthy_pod_eviction_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.unhealthy_pod_eviction_policy,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PodDisruptionBudgetSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
        > as crate::OptionableConvert>::merge(
            &mut self.max_unavailable,
            other.max_unavailable,
        )?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
        > as crate::OptionableConvert>::merge(
            &mut self.min_available,
            other.min_available,
        )?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
        > as crate::OptionableConvert>::merge(&mut self.selector, other.selector)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.unhealthy_pod_eviction_policy,
            other.unhealthy_pod_eviction_policy,
        )?;
        Ok(())
    }
}
