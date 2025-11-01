pub struct PodResourceClaimStatusAc {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub resource_claim_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PodResourceClaimStatus {
    type Optioned = PodResourceClaimStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PodResourceClaimStatusAc {
    type Optioned = PodResourceClaimStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PodResourceClaimStatus {
    fn into_optioned(self) -> PodResourceClaimStatusAc {
        PodResourceClaimStatusAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            resource_claim_name: crate::OptionableConvert::into_optioned(
                self.resource_claim_name,
            ),
        }
    }
    fn try_from_optioned(
        value: PodResourceClaimStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            resource_claim_name: crate::OptionableConvert::try_from_optioned(
                value.resource_claim_name,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PodResourceClaimStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.resource_claim_name,
            other.resource_claim_name,
        )?;
        Ok(())
    }
}
