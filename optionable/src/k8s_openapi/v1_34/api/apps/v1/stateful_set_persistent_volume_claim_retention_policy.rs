pub struct StatefulSetPersistentVolumeClaimRetentionPolicyAc {
    pub when_deleted: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub when_scaled: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy {
    type Optioned = StatefulSetPersistentVolumeClaimRetentionPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for StatefulSetPersistentVolumeClaimRetentionPolicyAc {
    type Optioned = StatefulSetPersistentVolumeClaimRetentionPolicyAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy {
    fn into_optioned(self) -> StatefulSetPersistentVolumeClaimRetentionPolicyAc {
        StatefulSetPersistentVolumeClaimRetentionPolicyAc {
            when_deleted: crate::OptionableConvert::into_optioned(self.when_deleted),
            when_scaled: crate::OptionableConvert::into_optioned(self.when_scaled),
        }
    }
    fn try_from_optioned(
        value: StatefulSetPersistentVolumeClaimRetentionPolicyAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            when_deleted: crate::OptionableConvert::try_from_optioned(
                value.when_deleted,
            )?,
            when_scaled: crate::OptionableConvert::try_from_optioned(value.when_scaled)?,
        })
    }
    fn merge(
        &mut self,
        other: StatefulSetPersistentVolumeClaimRetentionPolicyAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.when_deleted, other.when_deleted)?;
        crate::OptionableConvert::merge(&mut self.when_scaled, other.when_scaled)?;
        Ok(())
    }
}
