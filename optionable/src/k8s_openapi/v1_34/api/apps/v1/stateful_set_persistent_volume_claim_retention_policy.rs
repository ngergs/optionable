pub struct StatefulSetPersistentVolumeClaimRetentionPolicyOpt {
    pub when_deleted: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub when_scaled: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy {
    type Optioned = StatefulSetPersistentVolumeClaimRetentionPolicyOpt;
}
#[automatically_derived]
impl crate::Optionable for StatefulSetPersistentVolumeClaimRetentionPolicyOpt {
    type Optioned = StatefulSetPersistentVolumeClaimRetentionPolicyOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::apps::v1::StatefulSetPersistentVolumeClaimRetentionPolicy {
    fn into_optioned(self) -> StatefulSetPersistentVolumeClaimRetentionPolicyOpt {
        StatefulSetPersistentVolumeClaimRetentionPolicyOpt {
            when_deleted: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.when_deleted),
            when_scaled: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.when_scaled),
        }
    }
    fn try_from_optioned(
        value: StatefulSetPersistentVolumeClaimRetentionPolicyOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            when_deleted: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.when_deleted)?,
            when_scaled: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.when_scaled)?,
        })
    }
    fn merge(
        &mut self,
        other: StatefulSetPersistentVolumeClaimRetentionPolicyOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.when_deleted,
            other.when_deleted,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.when_scaled, other.when_scaled)?;
        Ok(())
    }
}
