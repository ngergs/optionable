pub struct CapacityRequirementsOpt {
    pub requests: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::CapacityRequirements {
    type Optioned = CapacityRequirementsOpt;
}
#[automatically_derived]
impl crate::Optionable for CapacityRequirementsOpt {
    type Optioned = CapacityRequirementsOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1::CapacityRequirements {
    fn into_optioned(self) -> CapacityRequirementsOpt {
        CapacityRequirementsOpt {
            requests: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::into_optioned(self.requests),
        }
    }
    fn try_from_optioned(
        value: CapacityRequirementsOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
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
        other: CapacityRequirementsOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            >,
        > as crate::OptionableConvert>::merge(&mut self.requests, other.requests)?;
        Ok(())
    }
}
