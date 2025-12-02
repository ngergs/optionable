#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct ServiceCIDRSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidrs: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1beta1::ServiceCIDRSpec {
    type Optioned = ServiceCIDRSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ServiceCIDRSpecAc {
    type Optioned = ServiceCIDRSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1beta1::ServiceCIDRSpec {
    fn into_optioned(self) -> ServiceCIDRSpecAc {
        ServiceCIDRSpecAc {
            cidrs: crate::OptionableConvert::into_optioned(self.cidrs),
        }
    }
    fn try_from_optioned(value: ServiceCIDRSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            cidrs: crate::OptionableConvert::try_from_optioned(value.cidrs)?,
        })
    }
    fn merge(&mut self, other: ServiceCIDRSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.cidrs, other.cidrs)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::networking::v1beta1::ServiceCIDRSpec>
for ServiceCIDRSpecAc {
    fn from_optionable(
        value: ::k8s_openapi::api::networking::v1beta1::ServiceCIDRSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::networking::v1beta1::ServiceCIDRSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::networking::v1beta1::ServiceCIDRSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
