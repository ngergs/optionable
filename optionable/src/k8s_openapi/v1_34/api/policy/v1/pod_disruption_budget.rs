#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    kube::Resource
)]
#[resource(inherit = PodDisruptionBudget)]
pub struct PodDisruptionBudgetAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: <Option<
        ::k8s_openapi::api::policy::v1::PodDisruptionBudgetSpec,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: <Option<
        ::k8s_openapi::api::policy::v1::PodDisruptionBudgetStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::policy::v1::PodDisruptionBudget {
    type Optioned = PodDisruptionBudgetAc;
}
#[automatically_derived]
impl crate::Optionable for PodDisruptionBudgetAc {
    type Optioned = PodDisruptionBudgetAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::policy::v1::PodDisruptionBudget {
    fn into_optioned(self) -> PodDisruptionBudgetAc {
        PodDisruptionBudgetAc {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(
        value: PodDisruptionBudgetAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(
        &mut self,
        other: PodDisruptionBudgetAc,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::policy::v1::PodDisruptionBudget;
