#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct PodDisruptionBudgetStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_healthy: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_healthy: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disrupted_pods: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disruptions_allowed: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_pods: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: <Option<i64> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::policy::v1::PodDisruptionBudgetStatus {
    type Optioned = PodDisruptionBudgetStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PodDisruptionBudgetStatusAc {
    type Optioned = PodDisruptionBudgetStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::policy::v1::PodDisruptionBudgetStatus {
    fn into_optioned(self) -> PodDisruptionBudgetStatusAc {
        PodDisruptionBudgetStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            current_healthy: Some(self.current_healthy),
            desired_healthy: Some(self.desired_healthy),
            disrupted_pods: crate::OptionableConvert::into_optioned(self.disrupted_pods),
            disruptions_allowed: Some(self.disruptions_allowed),
            expected_pods: Some(self.expected_pods),
            observed_generation: crate::OptionableConvert::into_optioned(
                self.observed_generation,
            ),
        }
    }
    fn try_from_optioned(
        value: PodDisruptionBudgetStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            current_healthy: value
                .current_healthy
                .ok_or(crate::optionable::Error {
                    missing_field: "current_healthy",
                })?,
            desired_healthy: value
                .desired_healthy
                .ok_or(crate::optionable::Error {
                    missing_field: "desired_healthy",
                })?,
            disrupted_pods: crate::OptionableConvert::try_from_optioned(
                value.disrupted_pods,
            )?,
            disruptions_allowed: value
                .disruptions_allowed
                .ok_or(crate::optionable::Error {
                    missing_field: "disruptions_allowed",
                })?,
            expected_pods: value
                .expected_pods
                .ok_or(crate::optionable::Error {
                    missing_field: "expected_pods",
                })?,
            observed_generation: crate::OptionableConvert::try_from_optioned(
                value.observed_generation,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PodDisruptionBudgetStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        if let Some(other_value) = other.current_healthy {
            self.current_healthy = other_value;
        }
        if let Some(other_value) = other.desired_healthy {
            self.desired_healthy = other_value;
        }
        crate::OptionableConvert::merge(&mut self.disrupted_pods, other.disrupted_pods)?;
        if let Some(other_value) = other.disruptions_allowed {
            self.disruptions_allowed = other_value;
        }
        if let Some(other_value) = other.expected_pods {
            self.expected_pods = other_value;
        }
        crate::OptionableConvert::merge(
            &mut self.observed_generation,
            other.observed_generation,
        )?;
        Ok(())
    }
}
