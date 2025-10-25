pub struct ResourceClaimOpt {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub request: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::resource_claim::ResourceClaim {
    type Optioned = ResourceClaimOpt;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimOpt {
    type Optioned = ResourceClaimOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::resource_claim::ResourceClaim {
    fn into_optioned(self) -> ResourceClaimOpt {
        ResourceClaimOpt {
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            request: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.request),
        }
    }
    fn try_from_optioned(
        value: ResourceClaimOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            request: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.request)?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceClaimOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.request, other.request)?;
        Ok(())
    }
}
