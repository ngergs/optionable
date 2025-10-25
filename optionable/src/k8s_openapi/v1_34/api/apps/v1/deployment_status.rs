pub struct DeploymentStatusOpt {
    pub available_replicas: <Option<i32> as crate::Optionable>::Optioned,
    pub collision_count: <Option<i32> as crate::Optionable>::Optioned,
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::apps::v1::DeploymentCondition>,
    > as crate::Optionable>::Optioned,
    pub observed_generation: <Option<i64> as crate::Optionable>::Optioned,
    pub ready_replicas: <Option<i32> as crate::Optionable>::Optioned,
    pub replicas: <Option<i32> as crate::Optionable>::Optioned,
    pub terminating_replicas: <Option<i32> as crate::Optionable>::Optioned,
    pub unavailable_replicas: <Option<i32> as crate::Optionable>::Optioned,
    pub updated_replicas: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::apps::v1::deployment_status::DeploymentStatus {
    type Optioned = DeploymentStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for DeploymentStatusOpt {
    type Optioned = DeploymentStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::apps::v1::deployment_status::DeploymentStatus {
    fn into_optioned(self) -> DeploymentStatusOpt {
        DeploymentStatusOpt {
            available_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.available_replicas),
            collision_count: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.collision_count),
            conditions: <Option<
                std::vec::Vec<::k8s_openapi::api::apps::v1::DeploymentCondition>,
            > as crate::OptionableConvert>::into_optioned(self.conditions),
            observed_generation: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(self.observed_generation),
            ready_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.ready_replicas),
            replicas: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.replicas),
            terminating_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.terminating_replicas),
            unavailable_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.unavailable_replicas),
            updated_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.updated_replicas),
        }
    }
    fn try_from_optioned(
        value: DeploymentStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            available_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.available_replicas)?,
            collision_count: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.collision_count)?,
            conditions: <Option<
                std::vec::Vec<::k8s_openapi::api::apps::v1::DeploymentCondition>,
            > as crate::OptionableConvert>::try_from_optioned(value.conditions)?,
            observed_generation: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(
                value.observed_generation,
            )?,
            ready_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.ready_replicas)?,
            replicas: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.replicas)?,
            terminating_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(
                value.terminating_replicas,
            )?,
            unavailable_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(
                value.unavailable_replicas,
            )?,
            updated_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.updated_replicas)?,
        })
    }
    fn merge(
        &mut self,
        other: DeploymentStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.available_replicas,
            other.available_replicas,
        )?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.collision_count,
            other.collision_count,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::apps::v1::DeploymentCondition>,
        > as crate::OptionableConvert>::merge(&mut self.conditions, other.conditions)?;
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
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(&mut self.replicas, other.replicas)?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.terminating_replicas,
            other.terminating_replicas,
        )?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.unavailable_replicas,
            other.unavailable_replicas,
        )?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.updated_replicas,
            other.updated_replicas,
        )?;
        Ok(())
    }
}
