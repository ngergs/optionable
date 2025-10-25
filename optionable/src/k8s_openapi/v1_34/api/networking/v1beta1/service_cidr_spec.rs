pub struct ServiceCIDRSpecOpt {
    pub cidrs: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::networking::v1beta1::service_cidr_spec::ServiceCIDRSpec {
    type Optioned = ServiceCIDRSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for ServiceCIDRSpecOpt {
    type Optioned = ServiceCIDRSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1beta1::service_cidr_spec::ServiceCIDRSpec {
    fn into_optioned(self) -> ServiceCIDRSpecOpt {
        ServiceCIDRSpecOpt {
            cidrs: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.cidrs),
        }
    }
    fn try_from_optioned(
        value: ServiceCIDRSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            cidrs: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.cidrs)?,
        })
    }
    fn merge(
        &mut self,
        other: ServiceCIDRSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.cidrs, other.cidrs)?;
        Ok(())
    }
}
