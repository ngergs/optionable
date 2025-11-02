#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
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
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1beta1::ServiceCIDRSpec {
    fn into_optioned(self) -> ServiceCIDRSpecAc {
        ServiceCIDRSpecAc {
            cidrs: crate::OptionableConvert::into_optioned(self.cidrs),
        }
    }
    fn try_from_optioned(
        value: ServiceCIDRSpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            cidrs: crate::OptionableConvert::try_from_optioned(value.cidrs)?,
        })
    }
    fn merge(
        &mut self,
        other: ServiceCIDRSpecAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.cidrs, other.cidrs)?;
        Ok(())
    }
}
