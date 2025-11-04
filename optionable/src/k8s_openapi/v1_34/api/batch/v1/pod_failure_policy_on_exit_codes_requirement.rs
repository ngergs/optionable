#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct PodFailurePolicyOnExitCodesRequirementAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<<std::vec::Vec<i32> as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::batch::v1::PodFailurePolicyOnExitCodesRequirement {
    type Optioned = PodFailurePolicyOnExitCodesRequirementAc;
}
#[automatically_derived]
impl crate::Optionable for PodFailurePolicyOnExitCodesRequirementAc {
    type Optioned = PodFailurePolicyOnExitCodesRequirementAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::batch::v1::PodFailurePolicyOnExitCodesRequirement {
    fn into_optioned(self) -> PodFailurePolicyOnExitCodesRequirementAc {
        PodFailurePolicyOnExitCodesRequirementAc {
            container_name: crate::OptionableConvert::into_optioned(self.container_name),
            operator: Some(crate::OptionableConvert::into_optioned(self.operator)),
            values: Some(crate::OptionableConvert::into_optioned(self.values)),
        }
    }
    fn try_from_optioned(
        value: PodFailurePolicyOnExitCodesRequirementAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            container_name: crate::OptionableConvert::try_from_optioned(
                value.container_name,
            )?,
            operator: crate::OptionableConvert::try_from_optioned(
                value
                    .operator
                    .ok_or(crate::optionable::Error {
                        missing_field: "operator",
                    })?,
            )?,
            values: crate::OptionableConvert::try_from_optioned(
                value
                    .values
                    .ok_or(crate::optionable::Error {
                        missing_field: "values",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PodFailurePolicyOnExitCodesRequirementAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.container_name, other.container_name)?;
        if let Some(other_value) = other.operator {
            crate::OptionableConvert::merge(&mut self.operator, other_value)?;
        }
        if let Some(other_value) = other.values {
            crate::OptionableConvert::merge(&mut self.values, other_value)?;
        }
        Ok(())
    }
}
