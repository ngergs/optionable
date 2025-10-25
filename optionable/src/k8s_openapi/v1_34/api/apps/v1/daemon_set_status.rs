pub struct DaemonSetStatusOpt {
    pub collision_count: <Option<i32> as crate::Optionable>::Optioned,
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::apps::v1::DaemonSetCondition>,
    > as crate::Optionable>::Optioned,
    pub current_number_scheduled: Option<i32>,
    pub desired_number_scheduled: Option<i32>,
    pub number_available: <Option<i32> as crate::Optionable>::Optioned,
    pub number_misscheduled: Option<i32>,
    pub number_ready: Option<i32>,
    pub number_unavailable: <Option<i32> as crate::Optionable>::Optioned,
    pub observed_generation: <Option<i64> as crate::Optionable>::Optioned,
    pub updated_number_scheduled: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::apps::v1::daemon_set_status::DaemonSetStatus {
    type Optioned = DaemonSetStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for DaemonSetStatusOpt {
    type Optioned = DaemonSetStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::apps::v1::daemon_set_status::DaemonSetStatus {
    fn into_optioned(self) -> DaemonSetStatusOpt {
        DaemonSetStatusOpt {
            collision_count: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.collision_count),
            conditions: <Option<
                std::vec::Vec<::k8s_openapi::api::apps::v1::DaemonSetCondition>,
            > as crate::OptionableConvert>::into_optioned(self.conditions),
            current_number_scheduled: Some(self.current_number_scheduled),
            desired_number_scheduled: Some(self.desired_number_scheduled),
            number_available: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.number_available),
            number_misscheduled: Some(self.number_misscheduled),
            number_ready: Some(self.number_ready),
            number_unavailable: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.number_unavailable),
            observed_generation: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(self.observed_generation),
            updated_number_scheduled: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.updated_number_scheduled),
        }
    }
    fn try_from_optioned(
        value: DaemonSetStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            collision_count: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.collision_count)?,
            conditions: <Option<
                std::vec::Vec<::k8s_openapi::api::apps::v1::DaemonSetCondition>,
            > as crate::OptionableConvert>::try_from_optioned(value.conditions)?,
            current_number_scheduled: value
                .current_number_scheduled
                .ok_or(crate::optionable::Error {
                    missing_field: "current_number_scheduled",
                })?,
            desired_number_scheduled: value
                .desired_number_scheduled
                .ok_or(crate::optionable::Error {
                    missing_field: "desired_number_scheduled",
                })?,
            number_available: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.number_available)?,
            number_misscheduled: value
                .number_misscheduled
                .ok_or(crate::optionable::Error {
                    missing_field: "number_misscheduled",
                })?,
            number_ready: value
                .number_ready
                .ok_or(crate::optionable::Error {
                    missing_field: "number_ready",
                })?,
            number_unavailable: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.number_unavailable)?,
            observed_generation: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(
                value.observed_generation,
            )?,
            updated_number_scheduled: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(
                value.updated_number_scheduled,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: DaemonSetStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.collision_count,
            other.collision_count,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::apps::v1::DaemonSetCondition>,
        > as crate::OptionableConvert>::merge(&mut self.conditions, other.conditions)?;
        if let Some(other_value) = other.current_number_scheduled {
            self.current_number_scheduled = other_value;
        }
        if let Some(other_value) = other.desired_number_scheduled {
            self.desired_number_scheduled = other_value;
        }
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.number_available,
            other.number_available,
        )?;
        if let Some(other_value) = other.number_misscheduled {
            self.number_misscheduled = other_value;
        }
        if let Some(other_value) = other.number_ready {
            self.number_ready = other_value;
        }
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.number_unavailable,
            other.number_unavailable,
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
            &mut self.updated_number_scheduled,
            other.updated_number_scheduled,
        )?;
        Ok(())
    }
}
