#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PodResourceClaimAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claim_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claim_template_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PodResourceClaim {
    type Optioned = PodResourceClaimAc;
}
#[automatically_derived]
impl crate::Optionable for PodResourceClaimAc {
    type Optioned = PodResourceClaimAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PodResourceClaim {
    fn into_optioned(self) -> PodResourceClaimAc {
        PodResourceClaimAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            resource_claim_name: crate::OptionableConvert::into_optioned(
                self.resource_claim_name,
            ),
            resource_claim_template_name: crate::OptionableConvert::into_optioned(
                self.resource_claim_template_name,
            ),
        }
    }
    fn try_from_optioned(
        value: PodResourceClaimAc,
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
            resource_claim_template_name: crate::OptionableConvert::try_from_optioned(
                value.resource_claim_template_name,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PodResourceClaimAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.resource_claim_name,
            other.resource_claim_name,
        )?;
        crate::OptionableConvert::merge(
            &mut self.resource_claim_template_name,
            other.resource_claim_template_name,
        )?;
        Ok(())
    }
}
