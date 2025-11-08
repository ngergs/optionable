#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_deadline_seconds: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: <Option<
        ::k8s_openapi::api::apps::v1::DeploymentStrategy,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<
        <::k8s_openapi::api::core::v1::PodTemplateSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::apps::v1::DeploymentSpec {
    type Optioned = DeploymentSpecAc;
}
#[automatically_derived]
impl crate::Optionable for DeploymentSpecAc {
    type Optioned = DeploymentSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::apps::v1::DeploymentSpec {
    fn into_optioned(self) -> DeploymentSpecAc {
        DeploymentSpecAc {
            min_ready_seconds: crate::OptionableConvert::into_optioned(
                self.min_ready_seconds,
            ),
            paused: crate::OptionableConvert::into_optioned(self.paused),
            progress_deadline_seconds: crate::OptionableConvert::into_optioned(
                self.progress_deadline_seconds,
            ),
            replicas: crate::OptionableConvert::into_optioned(self.replicas),
            revision_history_limit: crate::OptionableConvert::into_optioned(
                self.revision_history_limit,
            ),
            selector: Some(crate::OptionableConvert::into_optioned(self.selector)),
            strategy: crate::OptionableConvert::into_optioned(self.strategy),
            template: Some(crate::OptionableConvert::into_optioned(self.template)),
        }
    }
    fn try_from_optioned(value: DeploymentSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            min_ready_seconds: crate::OptionableConvert::try_from_optioned(
                value.min_ready_seconds,
            )?,
            paused: crate::OptionableConvert::try_from_optioned(value.paused)?,
            progress_deadline_seconds: crate::OptionableConvert::try_from_optioned(
                value.progress_deadline_seconds,
            )?,
            replicas: crate::OptionableConvert::try_from_optioned(value.replicas)?,
            revision_history_limit: crate::OptionableConvert::try_from_optioned(
                value.revision_history_limit,
            )?,
            selector: crate::OptionableConvert::try_from_optioned(
                value
                    .selector
                    .ok_or(crate::Error {
                        missing_field: "selector",
                    })?,
            )?,
            strategy: crate::OptionableConvert::try_from_optioned(value.strategy)?,
            template: crate::OptionableConvert::try_from_optioned(
                value
                    .template
                    .ok_or(crate::Error {
                        missing_field: "template",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: DeploymentSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.min_ready_seconds,
            other.min_ready_seconds,
        )?;
        crate::OptionableConvert::merge(&mut self.paused, other.paused)?;
        crate::OptionableConvert::merge(
            &mut self.progress_deadline_seconds,
            other.progress_deadline_seconds,
        )?;
        crate::OptionableConvert::merge(&mut self.replicas, other.replicas)?;
        crate::OptionableConvert::merge(
            &mut self.revision_history_limit,
            other.revision_history_limit,
        )?;
        if let Some(other_value) = other.selector {
            crate::OptionableConvert::merge(&mut self.selector, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.strategy, other.strategy)?;
        if let Some(other_value) = other.template {
            crate::OptionableConvert::merge(&mut self.template, other_value)?;
        }
        Ok(())
    }
}
