#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinals: <Option<
        ::k8s_openapi::api::apps::v1::StatefulSetOrdinals,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_volume_claim_retention_policy: <Option<
        ::k8s_openapi::api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_management_policy: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_history_limit: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<
        <::k8s_openapi::api::core::v1::PodTemplateSpec as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_strategy: <Option<
        ::k8s_openapi::api::apps::v1::StatefulSetUpdateStrategy,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_claim_templates: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::PersistentVolumeClaim>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::apps::v1::StatefulSetSpec {
    type Optioned = StatefulSetSpecAc;
}
#[automatically_derived]
impl crate::Optionable for StatefulSetSpecAc {
    type Optioned = StatefulSetSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::apps::v1::StatefulSetSpec {
    fn into_optioned(self) -> StatefulSetSpecAc {
        StatefulSetSpecAc {
            min_ready_seconds: crate::OptionableConvert::into_optioned(
                self.min_ready_seconds,
            ),
            ordinals: crate::OptionableConvert::into_optioned(self.ordinals),
            persistent_volume_claim_retention_policy: crate::OptionableConvert::into_optioned(
                self.persistent_volume_claim_retention_policy,
            ),
            pod_management_policy: crate::OptionableConvert::into_optioned(
                self.pod_management_policy,
            ),
            replicas: crate::OptionableConvert::into_optioned(self.replicas),
            revision_history_limit: crate::OptionableConvert::into_optioned(
                self.revision_history_limit,
            ),
            selector: Some(crate::OptionableConvert::into_optioned(self.selector)),
            service_name: crate::OptionableConvert::into_optioned(self.service_name),
            template: Some(crate::OptionableConvert::into_optioned(self.template)),
            update_strategy: crate::OptionableConvert::into_optioned(
                self.update_strategy,
            ),
            volume_claim_templates: crate::OptionableConvert::into_optioned(
                self.volume_claim_templates,
            ),
        }
    }
    fn try_from_optioned(value: StatefulSetSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            min_ready_seconds: crate::OptionableConvert::try_from_optioned(
                value.min_ready_seconds,
            )?,
            ordinals: crate::OptionableConvert::try_from_optioned(value.ordinals)?,
            persistent_volume_claim_retention_policy: crate::OptionableConvert::try_from_optioned(
                value.persistent_volume_claim_retention_policy,
            )?,
            pod_management_policy: crate::OptionableConvert::try_from_optioned(
                value.pod_management_policy,
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
            service_name: crate::OptionableConvert::try_from_optioned(
                value.service_name,
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
            volume_claim_templates: crate::OptionableConvert::try_from_optioned(
                value.volume_claim_templates,
            )?,
        })
    }
    fn merge(&mut self, other: StatefulSetSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.min_ready_seconds,
            other.min_ready_seconds,
        )?;
        crate::OptionableConvert::merge(&mut self.ordinals, other.ordinals)?;
        crate::OptionableConvert::merge(
            &mut self.persistent_volume_claim_retention_policy,
            other.persistent_volume_claim_retention_policy,
        )?;
        crate::OptionableConvert::merge(
            &mut self.pod_management_policy,
            other.pod_management_policy,
        )?;
        crate::OptionableConvert::merge(&mut self.replicas, other.replicas)?;
        crate::OptionableConvert::merge(
            &mut self.revision_history_limit,
            other.revision_history_limit,
        )?;
        if let Some(other_value) = other.selector {
            crate::OptionableConvert::merge(&mut self.selector, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.service_name, other.service_name)?;
        if let Some(other_value) = other.template {
            crate::OptionableConvert::merge(&mut self.template, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.update_strategy,
            other.update_strategy,
        )?;
        crate::OptionableConvert::merge(
            &mut self.volume_claim_templates,
            other.volume_claim_templates,
        )?;
        Ok(())
    }
}
