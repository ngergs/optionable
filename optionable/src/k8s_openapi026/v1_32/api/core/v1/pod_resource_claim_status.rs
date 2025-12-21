#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodResourceClaimStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claim_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::core::v1::PodResourceClaimStatus {
    type Optioned = PodResourceClaimStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PodResourceClaimStatusAc {
    type Optioned = PodResourceClaimStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::core::v1::PodResourceClaimStatus {
    fn into_optioned(self) -> PodResourceClaimStatusAc {
        PodResourceClaimStatusAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            resource_claim_name: crate::OptionableConvert::into_optioned(
                self.resource_claim_name,
            ),
        }
    }
    fn try_from_optioned(value: PodResourceClaimStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
            resource_claim_name: crate::OptionableConvert::try_from_optioned(
                value.resource_claim_name,
            )?,
        })
    }
    fn merge(&mut self, other: PodResourceClaimStatusAc) -> Result<(), crate::Error> {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::core::v1::PodResourceClaimStatus>
for PodResourceClaimStatusAc {
    fn from_optionable(
        value: k8s_openapi026::api::core::v1::PodResourceClaimStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::core::v1::PodResourceClaimStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::core::v1::PodResourceClaimStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
