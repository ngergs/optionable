#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DaemonSetStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_count: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi026::api::apps::v1::DaemonSetCondition>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_number_scheduled: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_number_scheduled: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_available: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_misscheduled: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_ready: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_unavailable: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: <Option<i64> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_number_scheduled: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::apps::v1::DaemonSetStatus {
    type Optioned = DaemonSetStatusAc;
}
#[automatically_derived]
impl crate::Optionable for DaemonSetStatusAc {
    type Optioned = DaemonSetStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::apps::v1::DaemonSetStatus {
    fn into_optioned(self) -> DaemonSetStatusAc {
        DaemonSetStatusAc {
            collision_count: crate::OptionableConvert::into_optioned(
                self.collision_count,
            ),
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            current_number_scheduled: Some(self.current_number_scheduled),
            desired_number_scheduled: Some(self.desired_number_scheduled),
            number_available: crate::OptionableConvert::into_optioned(
                self.number_available,
            ),
            number_misscheduled: Some(self.number_misscheduled),
            number_ready: Some(self.number_ready),
            number_unavailable: crate::OptionableConvert::into_optioned(
                self.number_unavailable,
            ),
            observed_generation: crate::OptionableConvert::into_optioned(
                self.observed_generation,
            ),
            updated_number_scheduled: crate::OptionableConvert::into_optioned(
                self.updated_number_scheduled,
            ),
        }
    }
    fn try_from_optioned(value: DaemonSetStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            collision_count: crate::OptionableConvert::try_from_optioned(
                value.collision_count,
            )?,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            current_number_scheduled: value
                .current_number_scheduled
                .ok_or(crate::Error {
                    missing_field: "current_number_scheduled",
                })?,
            desired_number_scheduled: value
                .desired_number_scheduled
                .ok_or(crate::Error {
                    missing_field: "desired_number_scheduled",
                })?,
            number_available: crate::OptionableConvert::try_from_optioned(
                value.number_available,
            )?,
            number_misscheduled: value
                .number_misscheduled
                .ok_or(crate::Error {
                    missing_field: "number_misscheduled",
                })?,
            number_ready: value
                .number_ready
                .ok_or(crate::Error {
                    missing_field: "number_ready",
                })?,
            number_unavailable: crate::OptionableConvert::try_from_optioned(
                value.number_unavailable,
            )?,
            observed_generation: crate::OptionableConvert::try_from_optioned(
                value.observed_generation,
            )?,
            updated_number_scheduled: crate::OptionableConvert::try_from_optioned(
                value.updated_number_scheduled,
            )?,
        })
    }
    fn merge(&mut self, other: DaemonSetStatusAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.collision_count,
            other.collision_count,
        )?;
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        if let Some(other_value) = other.current_number_scheduled {
            self.current_number_scheduled = other_value;
        }
        if let Some(other_value) = other.desired_number_scheduled {
            self.desired_number_scheduled = other_value;
        }
        crate::OptionableConvert::merge(
            &mut self.number_available,
            other.number_available,
        )?;
        if let Some(other_value) = other.number_misscheduled {
            self.number_misscheduled = other_value;
        }
        if let Some(other_value) = other.number_ready {
            self.number_ready = other_value;
        }
        crate::OptionableConvert::merge(
            &mut self.number_unavailable,
            other.number_unavailable,
        )?;
        crate::OptionableConvert::merge(
            &mut self.observed_generation,
            other.observed_generation,
        )?;
        crate::OptionableConvert::merge(
            &mut self.updated_number_scheduled,
            other.updated_number_scheduled,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::apps::v1::DaemonSetStatus>
for DaemonSetStatusAc {
    fn from_optionable(value: k8s_openapi026::api::apps::v1::DaemonSetStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::apps::v1::DaemonSetStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::apps::v1::DaemonSetStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
