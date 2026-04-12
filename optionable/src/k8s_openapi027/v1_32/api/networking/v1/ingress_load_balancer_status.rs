#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// IngressLoadBalancerStatus represents the status of a load-balancer.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IngressLoadBalancerStatusAc {
    /// ingress is a list containing ingress points for the load-balancer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::networking::v1::IngressLoadBalancerIngress as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::networking::v1::IngressLoadBalancerStatus {
    type Optioned = IngressLoadBalancerStatusAc;
}
#[automatically_derived]
impl crate::Optionable for IngressLoadBalancerStatusAc {
    type Optioned = IngressLoadBalancerStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::networking::v1::IngressLoadBalancerStatus {
    fn into_optioned(self) -> IngressLoadBalancerStatusAc {
        IngressLoadBalancerStatusAc {
            ingress: crate::OptionableConvert::into_optioned(self.ingress),
        }
    }
    fn try_from_optioned(
        value: IngressLoadBalancerStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            ingress: crate::OptionableConvert::try_from_optioned(value.ingress)?,
        })
    }
    fn merge(&mut self, other: IngressLoadBalancerStatusAc) -> Result<(), crate::Error> {
        if self.ingress.is_none() {
            self.ingress = crate::OptionableConvert::try_from_optioned(other.ingress)?;
        } else if let Some(self_value) = self.ingress.as_mut()
            && let Some(other_value) = other.ingress
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::networking::v1::IngressLoadBalancerStatus,
> for IngressLoadBalancerStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::networking::v1::IngressLoadBalancerStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::networking::v1::IngressLoadBalancerStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::IngressLoadBalancerStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
