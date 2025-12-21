#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeploymentStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_replicas: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_count: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi026::api::apps::v1::DeploymentCondition>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: <Option<i64> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_replicas: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminating_replicas: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable_replicas: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_replicas: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::apps::v1::DeploymentStatus {
    type Optioned = DeploymentStatusAc;
}
#[automatically_derived]
impl crate::Optionable for DeploymentStatusAc {
    type Optioned = DeploymentStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::apps::v1::DeploymentStatus {
    fn into_optioned(self) -> DeploymentStatusAc {
        DeploymentStatusAc {
            available_replicas: crate::OptionableConvert::into_optioned(
                self.available_replicas,
            ),
            collision_count: crate::OptionableConvert::into_optioned(
                self.collision_count,
            ),
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            observed_generation: crate::OptionableConvert::into_optioned(
                self.observed_generation,
            ),
            ready_replicas: crate::OptionableConvert::into_optioned(self.ready_replicas),
            replicas: crate::OptionableConvert::into_optioned(self.replicas),
            terminating_replicas: crate::OptionableConvert::into_optioned(
                self.terminating_replicas,
            ),
            unavailable_replicas: crate::OptionableConvert::into_optioned(
                self.unavailable_replicas,
            ),
            updated_replicas: crate::OptionableConvert::into_optioned(
                self.updated_replicas,
            ),
        }
    }
    fn try_from_optioned(value: DeploymentStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            available_replicas: crate::OptionableConvert::try_from_optioned(
                value.available_replicas,
            )?,
            collision_count: crate::OptionableConvert::try_from_optioned(
                value.collision_count,
            )?,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            observed_generation: crate::OptionableConvert::try_from_optioned(
                value.observed_generation,
            )?,
            ready_replicas: crate::OptionableConvert::try_from_optioned(
                value.ready_replicas,
            )?,
            replicas: crate::OptionableConvert::try_from_optioned(value.replicas)?,
            terminating_replicas: crate::OptionableConvert::try_from_optioned(
                value.terminating_replicas,
            )?,
            unavailable_replicas: crate::OptionableConvert::try_from_optioned(
                value.unavailable_replicas,
            )?,
            updated_replicas: crate::OptionableConvert::try_from_optioned(
                value.updated_replicas,
            )?,
        })
    }
    fn merge(&mut self, other: DeploymentStatusAc) -> Result<(), crate::Error> {
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
            &mut self.observed_generation,
            other.observed_generation,
        )?;
        crate::OptionableConvert::merge(&mut self.ready_replicas, other.ready_replicas)?;
        crate::OptionableConvert::merge(&mut self.replicas, other.replicas)?;
        crate::OptionableConvert::merge(
            &mut self.terminating_replicas,
            other.terminating_replicas,
        )?;
        crate::OptionableConvert::merge(
            &mut self.unavailable_replicas,
            other.unavailable_replicas,
        )?;
        crate::OptionableConvert::merge(
            &mut self.updated_replicas,
            other.updated_replicas,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::apps::v1::DeploymentStatus>
for DeploymentStatusAc {
    fn from_optionable(value: k8s_openapi026::api::apps::v1::DeploymentStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::apps::v1::DeploymentStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::apps::v1::DeploymentStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
