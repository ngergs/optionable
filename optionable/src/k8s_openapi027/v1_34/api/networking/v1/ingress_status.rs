#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// IngressStatus describe the current state of the Ingress.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IngressStatusAc {
    /// loadBalancer contains the current status of the load-balancer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<
        <::k8s_openapi027::api::networking::v1::IngressLoadBalancerStatus as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1::IngressStatus {
    type Optioned = IngressStatusAc;
}
#[automatically_derived]
impl crate::Optionable for IngressStatusAc {
    type Optioned = IngressStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::networking::v1::IngressStatus {
    fn into_optioned(self) -> IngressStatusAc {
        IngressStatusAc {
            load_balancer: crate::OptionableConvert::into_optioned(self.load_balancer),
        }
    }
    fn try_from_optioned(value: IngressStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            load_balancer: crate::OptionableConvert::try_from_optioned(
                value.load_balancer,
            )?,
        })
    }
    fn merge(&mut self, other: IngressStatusAc) -> Result<(), crate::Error> {
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
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1::IngressStatus>
for IngressStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::networking::v1::IngressStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::networking::v1::IngressStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::IngressStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for IngressStatusAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.load_balancer,
            other.load_balancer,
        );
    }
}
