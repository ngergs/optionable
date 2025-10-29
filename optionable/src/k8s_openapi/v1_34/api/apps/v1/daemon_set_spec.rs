pub struct DaemonSetSpecOpt {
    pub min_ready_seconds: <Option<i32> as crate::Optionable>::Optioned,
    pub revision_history_limit: <Option<i32> as crate::Optionable>::Optioned,
    pub selector: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    pub template: Option<
        <::k8s_openapi::api::core::v1::PodTemplateSpec as crate::Optionable>::Optioned,
    >,
    pub update_strategy: <Option<
        ::k8s_openapi::api::apps::v1::DaemonSetUpdateStrategy,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::apps::v1::DaemonSetSpec {
    type Optioned = DaemonSetSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for DaemonSetSpecOpt {
    type Optioned = DaemonSetSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::apps::v1::DaemonSetSpec {
    fn into_optioned(self) -> DaemonSetSpecOpt {
        DaemonSetSpecOpt {
            min_ready_seconds: crate::OptionableConvert::into_optioned(
                self.min_ready_seconds,
            ),
            revision_history_limit: crate::OptionableConvert::into_optioned(
                self.revision_history_limit,
            ),
            selector: Some(crate::OptionableConvert::into_optioned(self.selector)),
            template: Some(crate::OptionableConvert::into_optioned(self.template)),
            update_strategy: crate::OptionableConvert::into_optioned(
                self.update_strategy,
            ),
        }
    }
    fn try_from_optioned(
        value: DaemonSetSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            min_ready_seconds: crate::OptionableConvert::try_from_optioned(
                value.min_ready_seconds,
            )?,
            revision_history_limit: crate::OptionableConvert::try_from_optioned(
                value.revision_history_limit,
            )?,
            selector: crate::OptionableConvert::try_from_optioned(
                value
                    .selector
                    .ok_or(crate::optionable::Error {
                        missing_field: "selector",
                    })?,
            )?,
            template: crate::OptionableConvert::try_from_optioned(
                value
                    .template
                    .ok_or(crate::optionable::Error {
                        missing_field: "template",
                    })?,
            )?,
            update_strategy: crate::OptionableConvert::try_from_optioned(
                value.update_strategy,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: DaemonSetSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.min_ready_seconds,
            other.min_ready_seconds,
        )?;
        crate::OptionableConvert::merge(
            &mut self.revision_history_limit,
            other.revision_history_limit,
        )?;
        if let Some(other_value) = other.selector {
            crate::OptionableConvert::merge(&mut self.selector, other_value)?;
        }
        if let Some(other_value) = other.template {
            crate::OptionableConvert::merge(&mut self.template, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.update_strategy,
            other.update_strategy,
        )?;
        Ok(())
    }
}
