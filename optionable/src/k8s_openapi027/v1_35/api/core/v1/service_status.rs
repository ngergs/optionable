#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServiceStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Condition>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer: <Option<
        ::k8s_openapi027::api::core::v1::LoadBalancerStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ServiceStatus {
    type Optioned = ServiceStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ServiceStatusAc {
    type Optioned = ServiceStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ServiceStatus {
    fn into_optioned(self) -> ServiceStatusAc {
        ServiceStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            load_balancer: crate::OptionableConvert::into_optioned(self.load_balancer),
        }
    }
    fn try_from_optioned(value: ServiceStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            load_balancer: crate::OptionableConvert::try_from_optioned(
                value.load_balancer,
            )?,
        })
    }
    fn merge(&mut self, other: ServiceStatusAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        crate::OptionableConvert::merge(&mut self.load_balancer, other.load_balancer)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ServiceStatus>
for ServiceStatusAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::ServiceStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ServiceStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ServiceStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
