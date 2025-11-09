#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct CapacityRequirementsAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::CapacityRequirements {
    type Optioned = CapacityRequirementsAc;
}
#[automatically_derived]
impl crate::Optionable for CapacityRequirementsAc {
    type Optioned = CapacityRequirementsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1::CapacityRequirements {
    fn into_optioned(self) -> CapacityRequirementsAc {
        CapacityRequirementsAc {
            requests: crate::OptionableConvert::into_optioned(self.requests),
        }
    }
    fn try_from_optioned(value: CapacityRequirementsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            requests: crate::OptionableConvert::try_from_optioned(value.requests)?,
        })
    }
    fn merge(&mut self, other: CapacityRequirementsAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.requests, other.requests)?;
        Ok(())
    }
}
