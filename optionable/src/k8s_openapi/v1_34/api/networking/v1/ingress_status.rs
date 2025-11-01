pub struct IngressStatusAc {
    pub load_balancer: <Option<
        ::k8s_openapi::api::networking::v1::IngressLoadBalancerStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::IngressStatus {
    type Optioned = IngressStatusAc;
}
#[automatically_derived]
impl crate::Optionable for IngressStatusAc {
    type Optioned = IngressStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::IngressStatus {
    fn into_optioned(self) -> IngressStatusAc {
        IngressStatusAc {
            load_balancer: crate::OptionableConvert::into_optioned(self.load_balancer),
        }
    }
    fn try_from_optioned(
        value: IngressStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            load_balancer: crate::OptionableConvert::try_from_optioned(
                value.load_balancer,
            )?,
        })
    }
    fn merge(&mut self, other: IngressStatusAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.load_balancer, other.load_balancer)?;
        Ok(())
    }
}
