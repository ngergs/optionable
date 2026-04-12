#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Describes the state of a migration at a certain point.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MigrationConditionAc {
    /// The last time this condition was updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// A human readable message indicating details about the transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// The reason for the condition's last transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
    /// Status of the condition, one of True, False, Unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<std::string::String>,
    /// Type of the condition.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::storagemigration::v1alpha1::MigrationCondition {
    type Optioned = MigrationConditionAc;
}
#[automatically_derived]
impl crate::Optionable for MigrationConditionAc {
    type Optioned = MigrationConditionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::storagemigration::v1alpha1::MigrationCondition {
    fn into_optioned(self) -> MigrationConditionAc {
        MigrationConditionAc {
            last_update_time: crate::OptionableConvert::into_optioned(
                self.last_update_time,
            ),
            message: self.message,
            reason: self.reason,
            status: Some(self.status),
            type_: Some(self.type_),
        }
    }
    fn try_from_optioned(value: MigrationConditionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            last_update_time: crate::OptionableConvert::try_from_optioned(
                value.last_update_time,
            )?,
            message: value.message,
            reason: value.reason,
            status: value
                .status
                .ok_or(crate::Error {
                    missing_field: "status",
                })?,
            type_: value
                .type_
                .ok_or(crate::Error {
                    missing_field: "type_",
                })?,
        })
    }
    fn merge(&mut self, other: MigrationConditionAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.last_update_time,
            other.last_update_time,
        )?;
        if other.message.is_some() {
            self.message = other.message;
        }
        if other.reason.is_some() {
            self.reason = other.reason;
        }
        if let Some(other_value) = other.status {
            self.status = other_value;
        }
        if let Some(other_value) = other.type_ {
            self.type_ = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::storagemigration::v1alpha1::MigrationCondition,
> for MigrationConditionAc {
    fn from_optionable(
        value: k8s_openapi027::api::storagemigration::v1alpha1::MigrationCondition,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::storagemigration::v1alpha1::MigrationCondition,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::storagemigration::v1alpha1::MigrationCondition,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
