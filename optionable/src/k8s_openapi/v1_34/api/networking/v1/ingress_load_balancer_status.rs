#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IngressLoadBalancerStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress: <Option<
        std::vec::Vec<::k8s_openapi::api::networking::v1::IngressLoadBalancerIngress>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::networking::v1::IngressLoadBalancerStatus {
    type Optioned = IngressLoadBalancerStatusAc;
}
#[automatically_derived]
impl crate::Optionable for IngressLoadBalancerStatusAc {
    type Optioned = IngressLoadBalancerStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1::IngressLoadBalancerStatus {
    fn into_optioned(self) -> IngressLoadBalancerStatusAc {
        IngressLoadBalancerStatusAc {
            ingress: crate::OptionableConvert::into_optioned(self.ingress),
        }
    }
    fn try_from_optioned(
        value: IngressLoadBalancerStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            ingress: crate::OptionableConvert::try_from_optioned(value.ingress)?,
        })
    }
    fn merge(
        &mut self,
        other: IngressLoadBalancerStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.ingress, other.ingress)?;
        Ok(())
    }
}
