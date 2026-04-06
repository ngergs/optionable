#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StatefulSetPersistentVolumeClaimRetentionPolicyAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_deleted: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_scaled: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy {
    type Optioned = StatefulSetPersistentVolumeClaimRetentionPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for StatefulSetPersistentVolumeClaimRetentionPolicyAc {
    type Optioned = StatefulSetPersistentVolumeClaimRetentionPolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy {
    fn into_optioned(self) -> StatefulSetPersistentVolumeClaimRetentionPolicyAc {
        StatefulSetPersistentVolumeClaimRetentionPolicyAc {
            when_deleted: self.when_deleted,
            when_scaled: self.when_scaled,
        }
    }
    fn try_from_optioned(
        value: StatefulSetPersistentVolumeClaimRetentionPolicyAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            when_deleted: value.when_deleted,
            when_scaled: value.when_scaled,
        })
    }
    fn merge(
        &mut self,
        other: StatefulSetPersistentVolumeClaimRetentionPolicyAc,
    ) -> Result<(), crate::Error> {
        self.when_deleted = other.when_deleted;
        self.when_scaled = other.when_scaled;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy,
> for StatefulSetPersistentVolumeClaimRetentionPolicyAc {
    fn from_optionable(
        value: k8s_openapi027::api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
