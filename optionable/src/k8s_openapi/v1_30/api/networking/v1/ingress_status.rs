#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct IngressStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::IngressStatus {
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
        crate::OptionableConvert::merge(&mut self.load_balancer, other.load_balancer)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::networking::v1::IngressStatus>
for IngressStatusAc {
    fn from_optionable(
        value: ::k8s_openapi::api::networking::v1::IngressStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::networking::v1::IngressStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::networking::v1::IngressStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
