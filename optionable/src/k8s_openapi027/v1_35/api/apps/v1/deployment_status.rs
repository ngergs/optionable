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
    pub available_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::apps::v1::DeploymentCondition as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminating_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_replicas: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::apps::v1::DeploymentStatus {
    type Optioned = DeploymentStatusAc;
}
#[automatically_derived]
impl crate::Optionable for DeploymentStatusAc {
    type Optioned = DeploymentStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::apps::v1::DeploymentStatus {
    fn into_optioned(self) -> DeploymentStatusAc {
        DeploymentStatusAc {
            available_replicas: self.available_replicas,
            collision_count: self.collision_count,
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            observed_generation: self.observed_generation,
            ready_replicas: self.ready_replicas,
            replicas: self.replicas,
            terminating_replicas: self.terminating_replicas,
            unavailable_replicas: self.unavailable_replicas,
            updated_replicas: self.updated_replicas,
        }
    }
    fn try_from_optioned(value: DeploymentStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            available_replicas: value.available_replicas,
            collision_count: value.collision_count,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            observed_generation: value.observed_generation,
            ready_replicas: value.ready_replicas,
            replicas: value.replicas,
            terminating_replicas: value.terminating_replicas,
            unavailable_replicas: value.unavailable_replicas,
            updated_replicas: value.updated_replicas,
        })
    }
    fn merge(&mut self, other: DeploymentStatusAc) -> Result<(), crate::Error> {
        self.available_replicas = other.available_replicas;
        self.collision_count = other.collision_count;
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        self.observed_generation = other.observed_generation;
        self.ready_replicas = other.ready_replicas;
        self.replicas = other.replicas;
        self.terminating_replicas = other.terminating_replicas;
        self.unavailable_replicas = other.unavailable_replicas;
        self.updated_replicas = other.updated_replicas;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::apps::v1::DeploymentStatus>
for DeploymentStatusAc {
    fn from_optionable(value: k8s_openapi027::api::apps::v1::DeploymentStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::apps::v1::DeploymentStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apps::v1::DeploymentStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
