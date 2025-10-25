pub struct IngressLoadBalancerStatusOpt {
    pub ingress: <Option<
        std::vec::Vec<::k8s_openapi::api::networking::v1::IngressLoadBalancerIngress>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::networking::v1::ingress_load_balancer_status::IngressLoadBalancerStatus {
    type Optioned = IngressLoadBalancerStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for IngressLoadBalancerStatusOpt {
    type Optioned = IngressLoadBalancerStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1::ingress_load_balancer_status::IngressLoadBalancerStatus {
    fn into_optioned(self) -> IngressLoadBalancerStatusOpt {
        IngressLoadBalancerStatusOpt {
            ingress: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::networking::v1::IngressLoadBalancerIngress,
                >,
            > as crate::OptionableConvert>::into_optioned(self.ingress),
        }
    }
    fn try_from_optioned(
        value: IngressLoadBalancerStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            ingress: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::networking::v1::IngressLoadBalancerIngress,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.ingress)?,
        })
    }
    fn merge(
        &mut self,
        other: IngressLoadBalancerStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<::k8s_openapi::api::networking::v1::IngressLoadBalancerIngress>,
        > as crate::OptionableConvert>::merge(&mut self.ingress, other.ingress)?;
        Ok(())
    }
}
