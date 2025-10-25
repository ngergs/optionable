pub struct ReplicaSetStatusOpt {
    pub available_replicas: <Option<i32> as crate::Optionable>::Optioned,
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::apps::v1::ReplicaSetCondition>,
    > as crate::Optionable>::Optioned,
    pub fully_labeled_replicas: <Option<i32> as crate::Optionable>::Optioned,
    pub observed_generation: <Option<i64> as crate::Optionable>::Optioned,
    pub ready_replicas: <Option<i32> as crate::Optionable>::Optioned,
    pub replicas: Option<i32>,
    pub terminating_replicas: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::apps::v1::replica_set_status::ReplicaSetStatus {
    type Optioned = ReplicaSetStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for ReplicaSetStatusOpt {
    type Optioned = ReplicaSetStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::apps::v1::replica_set_status::ReplicaSetStatus {
    fn into_optioned(self) -> ReplicaSetStatusOpt {
        ReplicaSetStatusOpt {
            available_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.available_replicas),
            conditions: <Option<
                std::vec::Vec<::k8s_openapi::api::apps::v1::ReplicaSetCondition>,
            > as crate::OptionableConvert>::into_optioned(self.conditions),
            fully_labeled_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.fully_labeled_replicas),
            observed_generation: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(self.observed_generation),
            ready_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.ready_replicas),
            replicas: Some(self.replicas),
            terminating_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.terminating_replicas),
        }
    }
    fn try_from_optioned(
        value: ReplicaSetStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            available_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.available_replicas)?,
            conditions: <Option<
                std::vec::Vec<::k8s_openapi::api::apps::v1::ReplicaSetCondition>,
            > as crate::OptionableConvert>::try_from_optioned(value.conditions)?,
            fully_labeled_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(
                value.fully_labeled_replicas,
            )?,
            observed_generation: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(
                value.observed_generation,
            )?,
            ready_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.ready_replicas)?,
            replicas: value
                .replicas
                .ok_or(crate::optionable::Error {
                    missing_field: "replicas",
                })?,
            terminating_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(
                value.terminating_replicas,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ReplicaSetStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.available_replicas,
            other.available_replicas,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::apps::v1::ReplicaSetCondition>,
        > as crate::OptionableConvert>::merge(&mut self.conditions, other.conditions)?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.fully_labeled_replicas,
            other.fully_labeled_replicas,
        )?;
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(
            &mut self.observed_generation,
            other.observed_generation,
        )?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.ready_replicas,
            other.ready_replicas,
        )?;
        if let Some(other_value) = other.replicas {
            self.replicas = other_value;
        }
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.terminating_replicas,
            other.terminating_replicas,
        )?;
        Ok(())
    }
}
