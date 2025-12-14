#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LoadBalancerStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::LoadBalancerIngress>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::LoadBalancerStatus {
    type Optioned = LoadBalancerStatusAc;
}
#[automatically_derived]
impl crate::Optionable for LoadBalancerStatusAc {
    type Optioned = LoadBalancerStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::LoadBalancerStatus {
    fn into_optioned(self) -> LoadBalancerStatusAc {
        LoadBalancerStatusAc {
            ingress: crate::OptionableConvert::into_optioned(self.ingress),
        }
    }
    fn try_from_optioned(value: LoadBalancerStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            ingress: crate::OptionableConvert::try_from_optioned(value.ingress)?,
        })
    }
    fn merge(&mut self, other: LoadBalancerStatusAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.ingress, other.ingress)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::LoadBalancerStatus>
for LoadBalancerStatusAc {
    fn from_optionable(value: ::k8s_openapi::api::core::v1::LoadBalancerStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::LoadBalancerStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::LoadBalancerStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
