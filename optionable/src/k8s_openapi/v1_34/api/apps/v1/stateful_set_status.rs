pub struct StatefulSetStatusAc {
    pub available_replicas: <Option<i32> as crate::Optionable>::Optioned,
    pub collision_count: <Option<i32> as crate::Optionable>::Optioned,
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::apps::v1::StatefulSetCondition>,
    > as crate::Optionable>::Optioned,
    pub current_replicas: <Option<i32> as crate::Optionable>::Optioned,
    pub current_revision: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub observed_generation: <Option<i64> as crate::Optionable>::Optioned,
    pub ready_replicas: <Option<i32> as crate::Optionable>::Optioned,
    pub replicas: Option<i32>,
    pub update_revision: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub updated_replicas: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::apps::v1::StatefulSetStatus {
    type Optioned = StatefulSetStatusAc;
}
#[automatically_derived]
impl crate::Optionable for StatefulSetStatusAc {
    type Optioned = StatefulSetStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::apps::v1::StatefulSetStatus {
    fn into_optioned(self) -> StatefulSetStatusAc {
        StatefulSetStatusAc {
            available_replicas: crate::OptionableConvert::into_optioned(
                self.available_replicas,
            ),
            collision_count: crate::OptionableConvert::into_optioned(
                self.collision_count,
            ),
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            current_replicas: crate::OptionableConvert::into_optioned(
                self.current_replicas,
            ),
            current_revision: crate::OptionableConvert::into_optioned(
                self.current_revision,
            ),
            observed_generation: crate::OptionableConvert::into_optioned(
                self.observed_generation,
            ),
            ready_replicas: crate::OptionableConvert::into_optioned(self.ready_replicas),
            replicas: Some(self.replicas),
            update_revision: crate::OptionableConvert::into_optioned(
                self.update_revision,
            ),
            updated_replicas: crate::OptionableConvert::into_optioned(
                self.updated_replicas,
            ),
        }
    }
    fn try_from_optioned(
        value: StatefulSetStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            available_replicas: crate::OptionableConvert::try_from_optioned(
                value.available_replicas,
            )?,
            collision_count: crate::OptionableConvert::try_from_optioned(
                value.collision_count,
            )?,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            current_replicas: crate::OptionableConvert::try_from_optioned(
                value.current_replicas,
            )?,
            current_revision: crate::OptionableConvert::try_from_optioned(
                value.current_revision,
            )?,
            observed_generation: crate::OptionableConvert::try_from_optioned(
                value.observed_generation,
            )?,
            ready_replicas: crate::OptionableConvert::try_from_optioned(
                value.ready_replicas,
            )?,
            replicas: value
                .replicas
                .ok_or(crate::optionable::Error {
                    missing_field: "replicas",
                })?,
            update_revision: crate::OptionableConvert::try_from_optioned(
                value.update_revision,
            )?,
            updated_replicas: crate::OptionableConvert::try_from_optioned(
                value.updated_replicas,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: StatefulSetStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.available_replicas,
            other.available_replicas,
        )?;
        crate::OptionableConvert::merge(
            &mut self.collision_count,
            other.collision_count,
        )?;
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        crate::OptionableConvert::merge(
            &mut self.current_replicas,
            other.current_replicas,
        )?;
        crate::OptionableConvert::merge(
            &mut self.current_revision,
            other.current_revision,
        )?;
        crate::OptionableConvert::merge(
            &mut self.observed_generation,
            other.observed_generation,
        )?;
        crate::OptionableConvert::merge(&mut self.ready_replicas, other.ready_replicas)?;
        if let Some(other_value) = other.replicas {
            self.replicas = other_value;
        }
        crate::OptionableConvert::merge(
            &mut self.update_revision,
            other.update_revision,
        )?;
        crate::OptionableConvert::merge(
            &mut self.updated_replicas,
            other.updated_replicas,
        )?;
        Ok(())
    }
}
