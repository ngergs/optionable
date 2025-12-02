#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct HPAScalingPolicyAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_seconds: Option<i32>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::autoscaling::v2::HPAScalingPolicy {
    type Optioned = HPAScalingPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for HPAScalingPolicyAc {
    type Optioned = HPAScalingPolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::autoscaling::v2::HPAScalingPolicy {
    fn into_optioned(self) -> HPAScalingPolicyAc {
        HPAScalingPolicyAc {
            period_seconds: Some(self.period_seconds),
            type_: Some(crate::OptionableConvert::into_optioned(self.type_)),
            value: Some(self.value),
        }
    }
    fn try_from_optioned(value: HPAScalingPolicyAc) -> Result<Self, crate::Error> {
        Ok(Self {
            period_seconds: value
                .period_seconds
                .ok_or(crate::Error {
                    missing_field: "period_seconds",
                })?,
            type_: crate::OptionableConvert::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::Error {
                        missing_field: "type_",
                    })?,
            )?,
            value: value
                .value
                .ok_or(crate::Error {
                    missing_field: "value",
                })?,
        })
    }
    fn merge(&mut self, other: HPAScalingPolicyAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.period_seconds {
            self.period_seconds = other_value;
        }
        if let Some(other_value) = other.type_ {
            crate::OptionableConvert::merge(&mut self.type_, other_value)?;
        }
        if let Some(other_value) = other.value {
            self.value = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::autoscaling::v2::HPAScalingPolicy>
for HPAScalingPolicyAc {
    fn from_optionable(
        value: ::k8s_openapi::api::autoscaling::v2::HPAScalingPolicy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::autoscaling::v2::HPAScalingPolicy, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::autoscaling::v2::HPAScalingPolicy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
