#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ServiceStatus represents the current status of a service.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServiceStatusAc {
    /// Current service state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Condition as crate::Optionable>::Optioned,
        >,
    >,
    /// LoadBalancer contains the current status of the load-balancer, if one is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<
        <::k8s_openapi027::api::core::v1::LoadBalancerStatus as crate::Optionable>::Optioned,
    >,
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
        if self.conditions.is_none() {
            self.conditions = crate::OptionableConvert::try_from_optioned(
                other.conditions,
            )?;
        } else if let Some(self_value) = self.conditions.as_mut()
            && let Some(other_value) = other.conditions
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if self.load_balancer.is_none() {
            self.load_balancer = crate::OptionableConvert::try_from_optioned(
                other.load_balancer,
            )?;
        } else if let Some(self_value) = self.load_balancer.as_mut()
            && let Some(other_value) = other.load_balancer
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
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
impl k8s_openapi027::DeepMerge for ServiceStatusAc {
    fn merge_from(&mut self, other: Self) {
        crate::k8s_openapi::merge::merge_map_option_wrapped(
            &mut self.conditions,
            other.conditions,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.load_balancer,
            other.load_balancer,
        );
    }
}
