#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// StatefulSetPersistentVolumeClaimRetentionPolicy describes the policy used for PVCs created from the StatefulSet VolumeClaimTemplates.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StatefulSetPersistentVolumeClaimRetentionPolicyAc {
    /// WhenDeleted specifies what happens to PVCs created from StatefulSet VolumeClaimTemplates when the StatefulSet is deleted. The default policy of `Retain` causes PVCs to not be affected by StatefulSet deletion. The `Delete` policy causes those PVCs to be deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_deleted: Option<std::string::String>,
    /// WhenScaled specifies what happens to PVCs created from StatefulSet VolumeClaimTemplates when the StatefulSet is scaled down. The default policy of `Retain` causes PVCs to not be affected by a scaledown. The `Delete` policy causes the associated PVCs for any excess pods above the replica count to be deleted.
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
        if self.when_deleted.is_none() {
            self.when_deleted = crate::OptionableConvert::try_from_optioned(
                other.when_deleted,
            )?;
        } else if let Some(self_value) = self.when_deleted.as_mut()
            && let Some(other_value) = other.when_deleted
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.when_scaled.is_none() {
            self.when_scaled = crate::OptionableConvert::try_from_optioned(
                other.when_scaled,
            )?;
        } else if let Some(self_value) = self.when_scaled.as_mut()
            && let Some(other_value) = other.when_scaled
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
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
