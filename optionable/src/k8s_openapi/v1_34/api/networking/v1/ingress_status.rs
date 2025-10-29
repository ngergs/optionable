pub struct IngressStatusOpt {
    pub load_balancer: <Option<
        ::k8s_openapi::api::networking::v1::IngressLoadBalancerStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::IngressStatus {
    type Optioned = IngressStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for IngressStatusOpt {
    type Optioned = IngressStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::IngressStatus {
    fn into_optioned(self) -> IngressStatusOpt {
        IngressStatusOpt {
            load_balancer: crate::OptionableConvert::into_optioned(self.load_balancer),
        }
    }
    fn try_from_optioned(
        value: IngressStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            load_balancer: crate::OptionableConvert::try_from_optioned(
                value.load_balancer,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: IngressStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.load_balancer, other.load_balancer)?;
        Ok(())
    }
}
