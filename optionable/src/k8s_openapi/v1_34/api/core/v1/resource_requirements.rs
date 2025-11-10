#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRequirementsAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ResourceClaim>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ResourceRequirements {
    type Optioned = ResourceRequirementsAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceRequirementsAc {
    type Optioned = ResourceRequirementsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ResourceRequirements {
    fn into_optioned(self) -> ResourceRequirementsAc {
        ResourceRequirementsAc {
            claims: crate::OptionableConvert::into_optioned(self.claims),
            limits: crate::OptionableConvert::into_optioned(self.limits),
            requests: crate::OptionableConvert::into_optioned(self.requests),
        }
    }
    fn try_from_optioned(value: ResourceRequirementsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            claims: crate::OptionableConvert::try_from_optioned(value.claims)?,
            limits: crate::OptionableConvert::try_from_optioned(value.limits)?,
            requests: crate::OptionableConvert::try_from_optioned(value.requests)?,
        })
    }
    fn merge(&mut self, other: ResourceRequirementsAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.claims, other.claims)?;
        crate::OptionableConvert::merge(&mut self.limits, other.limits)?;
        crate::OptionableConvert::merge(&mut self.requests, other.requests)?;
        Ok(())
    }
}
