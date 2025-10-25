pub struct ServiceStatusOpt {
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>,
    > as crate::Optionable>::Optioned,
    pub load_balancer: <Option<
        ::k8s_openapi::api::core::v1::LoadBalancerStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::service_status::ServiceStatus {
    type Optioned = ServiceStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for ServiceStatusOpt {
    type Optioned = ServiceStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::service_status::ServiceStatus {
    fn into_optioned(self) -> ServiceStatusOpt {
        ServiceStatusOpt {
            conditions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition,
                >,
            > as crate::OptionableConvert>::into_optioned(self.conditions),
            load_balancer: <Option<
                ::k8s_openapi::api::core::v1::LoadBalancerStatus,
            > as crate::OptionableConvert>::into_optioned(self.load_balancer),
        }
    }
    fn try_from_optioned(
        value: ServiceStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.conditions)?,
            load_balancer: <Option<
                ::k8s_openapi::api::core::v1::LoadBalancerStatus,
            > as crate::OptionableConvert>::try_from_optioned(value.load_balancer)?,
        })
    }
    fn merge(
        &mut self,
        other: ServiceStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<::k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>,
        > as crate::OptionableConvert>::merge(&mut self.conditions, other.conditions)?;
        <Option<
            ::k8s_openapi::api::core::v1::LoadBalancerStatus,
        > as crate::OptionableConvert>::merge(
            &mut self.load_balancer,
            other.load_balancer,
        )?;
        Ok(())
    }
}
