pub struct DeploymentSpecOpt {
    pub min_ready_seconds: <Option<i32> as crate::Optionable>::Optioned,
    pub paused: <Option<bool> as crate::Optionable>::Optioned,
    pub progress_deadline_seconds: <Option<i32> as crate::Optionable>::Optioned,
    pub replicas: <Option<i32> as crate::Optionable>::Optioned,
    pub revision_history_limit: <Option<i32> as crate::Optionable>::Optioned,
    pub selector: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    pub strategy: <Option<
        ::k8s_openapi::api::apps::v1::DeploymentStrategy,
    > as crate::Optionable>::Optioned,
    pub template: Option<
        <::k8s_openapi::api::core::v1::PodTemplateSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::apps::v1::DeploymentSpec {
    type Optioned = DeploymentSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for DeploymentSpecOpt {
    type Optioned = DeploymentSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::apps::v1::DeploymentSpec {
    fn into_optioned(self) -> DeploymentSpecOpt {
        DeploymentSpecOpt {
            min_ready_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.min_ready_seconds),
            paused: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.paused),
            progress_deadline_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(
                self.progress_deadline_seconds,
            ),
            replicas: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.replicas),
            revision_history_limit: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.revision_history_limit),
            selector: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::OptionableConvert>::into_optioned(
                    self.selector,
                ),
            ),
            strategy: <Option<
                ::k8s_openapi::api::apps::v1::DeploymentStrategy,
            > as crate::OptionableConvert>::into_optioned(self.strategy),
            template: Some(
                <::k8s_openapi::api::core::v1::PodTemplateSpec as crate::OptionableConvert>::into_optioned(
                    self.template,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: DeploymentSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            min_ready_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.min_ready_seconds)?,
            paused: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.paused)?,
            progress_deadline_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(
                value.progress_deadline_seconds,
            )?,
            replicas: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.replicas)?,
            revision_history_limit: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(
                value.revision_history_limit,
            )?,
            selector: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::OptionableConvert>::try_from_optioned(
                value
                    .selector
                    .ok_or(crate::optionable::Error {
                        missing_field: "selector",
                    })?,
            )?,
            strategy: <Option<
                ::k8s_openapi::api::apps::v1::DeploymentStrategy,
            > as crate::OptionableConvert>::try_from_optioned(value.strategy)?,
            template: <::k8s_openapi::api::core::v1::PodTemplateSpec as crate::OptionableConvert>::try_from_optioned(
                value
                    .template
                    .ok_or(crate::optionable::Error {
                        missing_field: "template",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: DeploymentSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.min_ready_seconds,
            other.min_ready_seconds,
        )?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.paused, other.paused)?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.progress_deadline_seconds,
            other.progress_deadline_seconds,
        )?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(&mut self.replicas, other.replicas)?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.revision_history_limit,
            other.revision_history_limit,
        )?;
        if let Some(other_value) = other.selector {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::OptionableConvert>::merge(
                &mut self.selector,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::api::apps::v1::DeploymentStrategy,
        > as crate::OptionableConvert>::merge(&mut self.strategy, other.strategy)?;
        if let Some(other_value) = other.template {
            <::k8s_openapi::api::core::v1::PodTemplateSpec as crate::OptionableConvert>::merge(
                &mut self.template,
                other_value,
            )?;
        }
        Ok(())
    }
}
