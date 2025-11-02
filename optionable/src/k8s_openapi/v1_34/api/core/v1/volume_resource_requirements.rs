#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct VolumeResourceRequirementsAc {
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
impl crate::Optionable for ::k8s_openapi::api::core::v1::VolumeResourceRequirements {
    type Optioned = VolumeResourceRequirementsAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeResourceRequirementsAc {
    type Optioned = VolumeResourceRequirementsAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::VolumeResourceRequirements {
    fn into_optioned(self) -> VolumeResourceRequirementsAc {
        VolumeResourceRequirementsAc {
            limits: crate::OptionableConvert::into_optioned(self.limits),
            requests: crate::OptionableConvert::into_optioned(self.requests),
        }
    }
    fn try_from_optioned(
        value: VolumeResourceRequirementsAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            limits: crate::OptionableConvert::try_from_optioned(value.limits)?,
            requests: crate::OptionableConvert::try_from_optioned(value.requests)?,
        })
    }
    fn merge(
        &mut self,
        other: VolumeResourceRequirementsAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.limits, other.limits)?;
        crate::OptionableConvert::merge(&mut self.requests, other.requests)?;
        Ok(())
    }
}
