pub struct ServiceStatusAc {
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>,
    > as crate::Optionable>::Optioned,
    pub load_balancer: <Option<
        ::k8s_openapi::api::core::v1::LoadBalancerStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ServiceStatus {
    type Optioned = ServiceStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ServiceStatusAc {
    type Optioned = ServiceStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ServiceStatus {
    fn into_optioned(self) -> ServiceStatusAc {
        ServiceStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            load_balancer: crate::OptionableConvert::into_optioned(self.load_balancer),
        }
    }
    fn try_from_optioned(
        value: ServiceStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            load_balancer: crate::OptionableConvert::try_from_optioned(
                value.load_balancer,
            )?,
        })
    }
    fn merge(&mut self, other: ServiceStatusAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        crate::OptionableConvert::merge(&mut self.load_balancer, other.load_balancer)?;
        Ok(())
    }
}
