pub struct ResourceRequirementsOpt {
    pub claims: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ResourceClaim>,
    > as crate::Optionable>::Optioned,
    pub limits: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    pub requests: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ResourceRequirements {
    type Optioned = ResourceRequirementsOpt;
}
#[automatically_derived]
impl crate::Optionable for ResourceRequirementsOpt {
    type Optioned = ResourceRequirementsOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ResourceRequirements {
    fn into_optioned(self) -> ResourceRequirementsOpt {
        ResourceRequirementsOpt {
            claims: crate::OptionableConvert::into_optioned(self.claims),
            limits: crate::OptionableConvert::into_optioned(self.limits),
            requests: crate::OptionableConvert::into_optioned(self.requests),
        }
    }
    fn try_from_optioned(
        value: ResourceRequirementsOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            claims: crate::OptionableConvert::try_from_optioned(value.claims)?,
            limits: crate::OptionableConvert::try_from_optioned(value.limits)?,
            requests: crate::OptionableConvert::try_from_optioned(value.requests)?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceRequirementsOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.claims, other.claims)?;
        crate::OptionableConvert::merge(&mut self.limits, other.limits)?;
        crate::OptionableConvert::merge(&mut self.requests, other.requests)?;
        Ok(())
    }
}
