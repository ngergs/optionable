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
impl crate::Optionable
for ::k8s_openapi::api::core::v1::resource_requirements::ResourceRequirements {
    type Optioned = ResourceRequirementsOpt;
}
#[automatically_derived]
impl crate::Optionable for ResourceRequirementsOpt {
    type Optioned = ResourceRequirementsOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::resource_requirements::ResourceRequirements {
    fn into_optioned(self) -> ResourceRequirementsOpt {
        ResourceRequirementsOpt {
            claims: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::ResourceClaim>,
            > as crate::OptionableConvert>::into_optioned(self.claims),
            limits: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::into_optioned(self.limits),
            requests: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::into_optioned(self.requests),
        }
    }
    fn try_from_optioned(
        value: ResourceRequirementsOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            claims: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::ResourceClaim>,
            > as crate::OptionableConvert>::try_from_optioned(value.claims)?,
            limits: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.limits)?,
            requests: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.requests)?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceRequirementsOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::ResourceClaim>,
        > as crate::OptionableConvert>::merge(&mut self.claims, other.claims)?;
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            >,
        > as crate::OptionableConvert>::merge(&mut self.limits, other.limits)?;
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            >,
        > as crate::OptionableConvert>::merge(&mut self.requests, other.requests)?;
        Ok(())
    }
}
