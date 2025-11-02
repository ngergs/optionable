#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplicationControllerStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_replicas: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ReplicationControllerCondition>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fully_labeled_replicas: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: <Option<i64> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_replicas: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ReplicationControllerStatus {
    type Optioned = ReplicationControllerStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ReplicationControllerStatusAc {
    type Optioned = ReplicationControllerStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::ReplicationControllerStatus {
    fn into_optioned(self) -> ReplicationControllerStatusAc {
        ReplicationControllerStatusAc {
            available_replicas: crate::OptionableConvert::into_optioned(
                self.available_replicas,
            ),
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            fully_labeled_replicas: crate::OptionableConvert::into_optioned(
                self.fully_labeled_replicas,
            ),
            observed_generation: crate::OptionableConvert::into_optioned(
                self.observed_generation,
            ),
            ready_replicas: crate::OptionableConvert::into_optioned(self.ready_replicas),
            replicas: Some(self.replicas),
        }
    }
    fn try_from_optioned(
        value: ReplicationControllerStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            available_replicas: crate::OptionableConvert::try_from_optioned(
                value.available_replicas,
            )?,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            fully_labeled_replicas: crate::OptionableConvert::try_from_optioned(
                value.fully_labeled_replicas,
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
        })
    }
    fn merge(
        &mut self,
        other: ReplicationControllerStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.available_replicas,
            other.available_replicas,
        )?;
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        crate::OptionableConvert::merge(
            &mut self.fully_labeled_replicas,
            other.fully_labeled_replicas,
        )?;
        crate::OptionableConvert::merge(
            &mut self.observed_generation,
            other.observed_generation,
        )?;
        crate::OptionableConvert::merge(&mut self.ready_replicas, other.ready_replicas)?;
        if let Some(other_value) = other.replicas {
            self.replicas = other_value;
        }
        Ok(())
    }
}
