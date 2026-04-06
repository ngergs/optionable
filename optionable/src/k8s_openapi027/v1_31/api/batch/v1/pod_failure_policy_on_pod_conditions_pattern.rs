#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodFailurePolicyOnPodConditionsPatternAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::batch::v1::PodFailurePolicyOnPodConditionsPattern {
    type Optioned = PodFailurePolicyOnPodConditionsPatternAc;
}
#[automatically_derived]
impl crate::Optionable for PodFailurePolicyOnPodConditionsPatternAc {
    type Optioned = PodFailurePolicyOnPodConditionsPatternAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::batch::v1::PodFailurePolicyOnPodConditionsPattern {
    fn into_optioned(self) -> PodFailurePolicyOnPodConditionsPatternAc {
        PodFailurePolicyOnPodConditionsPatternAc {
            status: Some(self.status),
            type_: Some(self.type_),
        }
    }
    fn try_from_optioned(
        value: PodFailurePolicyOnPodConditionsPatternAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            status: value
                .status
                .ok_or(crate::Error {
                    missing_field: "status",
                })?,
            type_: value
                .type_
                .ok_or(crate::Error {
                    missing_field: "type_",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: PodFailurePolicyOnPodConditionsPatternAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.status {
            self.status = other_value;
        }
        if let Some(other_value) = other.type_ {
            self.type_ = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::batch::v1::PodFailurePolicyOnPodConditionsPattern,
> for PodFailurePolicyOnPodConditionsPatternAc {
    fn from_optionable(
        value: k8s_openapi027::api::batch::v1::PodFailurePolicyOnPodConditionsPattern,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::batch::v1::PodFailurePolicyOnPodConditionsPattern,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::batch::v1::PodFailurePolicyOnPodConditionsPattern,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
