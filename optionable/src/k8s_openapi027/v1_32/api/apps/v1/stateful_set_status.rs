#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StatefulSetStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::apps::v1::StatefulSetCondition as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revision: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_revision: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_replicas: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::apps::v1::StatefulSetStatus {
    type Optioned = StatefulSetStatusAc;
}
#[automatically_derived]
impl crate::Optionable for StatefulSetStatusAc {
    type Optioned = StatefulSetStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::apps::v1::StatefulSetStatus {
    fn into_optioned(self) -> StatefulSetStatusAc {
        StatefulSetStatusAc {
            available_replicas: self.available_replicas,
            collision_count: self.collision_count,
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            current_replicas: self.current_replicas,
            current_revision: self.current_revision,
            observed_generation: self.observed_generation,
            ready_replicas: self.ready_replicas,
            replicas: Some(self.replicas),
            update_revision: self.update_revision,
            updated_replicas: self.updated_replicas,
        }
    }
    fn try_from_optioned(value: StatefulSetStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            available_replicas: value.available_replicas,
            collision_count: value.collision_count,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            current_replicas: value.current_replicas,
            current_revision: value.current_revision,
            observed_generation: value.observed_generation,
            ready_replicas: value.ready_replicas,
            replicas: value
                .replicas
                .ok_or(crate::Error {
                    missing_field: "replicas",
                })?,
            update_revision: value.update_revision,
            updated_replicas: value.updated_replicas,
        })
    }
    fn merge(&mut self, other: StatefulSetStatusAc) -> Result<(), crate::Error> {
        self.available_replicas = other.available_replicas;
        self.collision_count = other.collision_count;
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        self.current_replicas = other.current_replicas;
        self.current_revision = other.current_revision;
        self.observed_generation = other.observed_generation;
        self.ready_replicas = other.ready_replicas;
        if let Some(other_value) = other.replicas {
            self.replicas = other_value;
        }
        self.update_revision = other.update_revision;
        self.updated_replicas = other.updated_replicas;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::apps::v1::StatefulSetStatus>
for StatefulSetStatusAc {
    fn from_optionable(value: k8s_openapi027::api::apps::v1::StatefulSetStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::apps::v1::StatefulSetStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apps::v1::StatefulSetStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
