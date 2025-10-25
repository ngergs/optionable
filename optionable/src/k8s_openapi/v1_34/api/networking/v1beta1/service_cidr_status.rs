pub struct ServiceCIDRStatusOpt {
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1beta1::ServiceCIDRStatus {
    type Optioned = ServiceCIDRStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for ServiceCIDRStatusOpt {
    type Optioned = ServiceCIDRStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1beta1::ServiceCIDRStatus {
    fn into_optioned(self) -> ServiceCIDRStatusOpt {
        ServiceCIDRStatusOpt {
            conditions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition,
                >,
            > as crate::OptionableConvert>::into_optioned(self.conditions),
        }
    }
    fn try_from_optioned(
        value: ServiceCIDRStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.conditions)?,
        })
    }
    fn merge(
        &mut self,
        other: ServiceCIDRStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<::k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>,
        > as crate::OptionableConvert>::merge(&mut self.conditions, other.conditions)?;
        Ok(())
    }
}
