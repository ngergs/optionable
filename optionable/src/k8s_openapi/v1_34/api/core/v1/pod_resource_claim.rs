pub struct PodResourceClaimOpt {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub resource_claim_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub resource_claim_template_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::pod_resource_claim::PodResourceClaim {
    type Optioned = PodResourceClaimOpt;
}
#[automatically_derived]
impl crate::Optionable for PodResourceClaimOpt {
    type Optioned = PodResourceClaimOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::pod_resource_claim::PodResourceClaim {
    fn into_optioned(self) -> PodResourceClaimOpt {
        PodResourceClaimOpt {
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            resource_claim_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.resource_claim_name),
            resource_claim_template_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(
                self.resource_claim_template_name,
            ),
        }
    }
    fn try_from_optioned(
        value: PodResourceClaimOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            resource_claim_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.resource_claim_name,
            )?,
            resource_claim_template_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.resource_claim_template_name,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PodResourceClaimOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.resource_claim_name,
            other.resource_claim_name,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.resource_claim_template_name,
            other.resource_claim_template_name,
        )?;
        Ok(())
    }
}
