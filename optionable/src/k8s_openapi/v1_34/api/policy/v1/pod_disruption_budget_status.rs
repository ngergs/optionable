pub struct PodDisruptionBudgetStatusOpt {
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>,
    > as crate::Optionable>::Optioned,
    pub current_healthy: Option<i32>,
    pub desired_healthy: Option<i32>,
    pub disrupted_pods: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
        >,
    > as crate::Optionable>::Optioned,
    pub disruptions_allowed: Option<i32>,
    pub expected_pods: Option<i32>,
    pub observed_generation: <Option<i64> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::policy::v1::pod_disruption_budget_status::PodDisruptionBudgetStatus {
    type Optioned = PodDisruptionBudgetStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for PodDisruptionBudgetStatusOpt {
    type Optioned = PodDisruptionBudgetStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::policy::v1::pod_disruption_budget_status::PodDisruptionBudgetStatus {
    fn into_optioned(self) -> PodDisruptionBudgetStatusOpt {
        PodDisruptionBudgetStatusOpt {
            conditions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition,
                >,
            > as crate::OptionableConvert>::into_optioned(self.conditions),
            current_healthy: Some(self.current_healthy),
            desired_healthy: Some(self.desired_healthy),
            disrupted_pods: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
                >,
            > as crate::OptionableConvert>::into_optioned(self.disrupted_pods),
            disruptions_allowed: Some(self.disruptions_allowed),
            expected_pods: Some(self.expected_pods),
            observed_generation: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(self.observed_generation),
        }
    }
    fn try_from_optioned(
        value: PodDisruptionBudgetStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.conditions)?,
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
            disrupted_pods: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.disrupted_pods)?,
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
            observed_generation: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(value.observed_generation)?,
        })
    }
    fn merge(
        &mut self,
        other: PodDisruptionBudgetStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<::k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>,
        > as crate::OptionableConvert>::merge(&mut self.conditions, other.conditions)?;
        if let Some(other_value) = other.current_healthy {
            self.current_healthy = other_value;
        }
        if let Some(other_value) = other.desired_healthy {
            self.desired_healthy = other_value;
        }
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.disrupted_pods,
            other.disrupted_pods,
        )?;
        if let Some(other_value) = other.disruptions_allowed {
            self.disruptions_allowed = other_value;
        }
        if let Some(other_value) = other.expected_pods {
            self.expected_pods = other_value;
        }
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(
            &mut self.observed_generation,
            other.observed_generation,
        )?;
        Ok(())
    }
}
