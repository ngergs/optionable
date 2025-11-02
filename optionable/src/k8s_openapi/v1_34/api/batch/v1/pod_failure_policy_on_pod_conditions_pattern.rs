#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodFailurePolicyOnPodConditionsPatternAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::batch::v1::PodFailurePolicyOnPodConditionsPattern {
    type Optioned = PodFailurePolicyOnPodConditionsPatternAc;
}
#[automatically_derived]
impl crate::Optionable for PodFailurePolicyOnPodConditionsPatternAc {
    type Optioned = PodFailurePolicyOnPodConditionsPatternAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::batch::v1::PodFailurePolicyOnPodConditionsPattern {
    fn into_optioned(self) -> PodFailurePolicyOnPodConditionsPatternAc {
        PodFailurePolicyOnPodConditionsPatternAc {
            status: Some(crate::OptionableConvert::into_optioned(self.status)),
            type_: Some(crate::OptionableConvert::into_optioned(self.type_)),
        }
    }
    fn try_from_optioned(
        value: PodFailurePolicyOnPodConditionsPatternAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            status: crate::OptionableConvert::try_from_optioned(
                value
                    .status
                    .ok_or(crate::optionable::Error {
                        missing_field: "status",
                    })?,
            )?,
            type_: crate::OptionableConvert::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::optionable::Error {
                        missing_field: "type_",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PodFailurePolicyOnPodConditionsPatternAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.status {
            crate::OptionableConvert::merge(&mut self.status, other_value)?;
        }
        if let Some(other_value) = other.type_ {
            crate::OptionableConvert::merge(&mut self.type_, other_value)?;
        }
        Ok(())
    }
}
