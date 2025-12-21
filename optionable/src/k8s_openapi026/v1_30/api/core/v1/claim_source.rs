#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClaimSourceAc {
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
impl crate::Optionable for k8s_openapi026::api::core::v1::ClaimSource {
    type Optioned = ClaimSourceAc;
}
#[automatically_derived]
impl crate::Optionable for ClaimSourceAc {
    type Optioned = ClaimSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::core::v1::ClaimSource {
    fn into_optioned(self) -> ClaimSourceAc {
        ClaimSourceAc {
            resource_claim_name: crate::OptionableConvert::into_optioned(
                self.resource_claim_name,
            ),
            resource_claim_template_name: crate::OptionableConvert::into_optioned(
                self.resource_claim_template_name,
            ),
        }
    }
    fn try_from_optioned(value: ClaimSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            resource_claim_name: crate::OptionableConvert::try_from_optioned(
                value.resource_claim_name,
            )?,
            resource_claim_template_name: crate::OptionableConvert::try_from_optioned(
                value.resource_claim_template_name,
            )?,
        })
    }
    fn merge(&mut self, other: ClaimSourceAc) -> Result<(), crate::Error> {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::core::v1::ClaimSource>
for ClaimSourceAc {
    fn from_optionable(value: k8s_openapi026::api::core::v1::ClaimSource) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::core::v1::ClaimSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::core::v1::ClaimSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
