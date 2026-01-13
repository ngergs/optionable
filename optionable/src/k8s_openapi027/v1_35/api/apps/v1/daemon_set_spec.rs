#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DaemonSetSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<
        <::k8s_openapi027::api::core::v1::PodTemplateSpec as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_strategy: <Option<
        ::k8s_openapi027::api::apps::v1::DaemonSetUpdateStrategy,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::apps::v1::DaemonSetSpec {
    type Optioned = DaemonSetSpecAc;
}
#[automatically_derived]
impl crate::Optionable for DaemonSetSpecAc {
    type Optioned = DaemonSetSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::apps::v1::DaemonSetSpec {
    fn into_optioned(self) -> DaemonSetSpecAc {
        DaemonSetSpecAc {
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
    fn try_from_optioned(value: DaemonSetSpecAc) -> Result<Self, crate::Error> {
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
                    .ok_or(crate::Error {
                        missing_field: "selector",
                    })?,
            )?,
            template: crate::OptionableConvert::try_from_optioned(
                value
                    .template
                    .ok_or(crate::Error {
                        missing_field: "template",
                    })?,
            )?,
            update_strategy: crate::OptionableConvert::try_from_optioned(
                value.update_strategy,
            )?,
        })
    }
    fn merge(&mut self, other: DaemonSetSpecAc) -> Result<(), crate::Error> {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::apps::v1::DaemonSetSpec>
for DaemonSetSpecAc {
    fn from_optionable(value: k8s_openapi027::api::apps::v1::DaemonSetSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::apps::v1::DaemonSetSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apps::v1::DaemonSetSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
