pub struct LoadBalancerStatusOpt {
    pub ingress: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::LoadBalancerIngress>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::LoadBalancerStatus {
    type Optioned = LoadBalancerStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for LoadBalancerStatusOpt {
    type Optioned = LoadBalancerStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::LoadBalancerStatus {
    fn into_optioned(self) -> LoadBalancerStatusOpt {
        LoadBalancerStatusOpt {
            ingress: crate::OptionableConvert::into_optioned(self.ingress),
        }
    }
    fn try_from_optioned(
        value: LoadBalancerStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            ingress: crate::OptionableConvert::try_from_optioned(value.ingress)?,
        })
    }
    fn merge(
        &mut self,
        other: LoadBalancerStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.ingress, other.ingress)?;
        Ok(())
    }
}
