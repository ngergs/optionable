#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StorageVersionConditionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: <Option<
        ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: <Option<i64> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::apiserverinternal::v1alpha1::StorageVersionCondition {
    type Optioned = StorageVersionConditionAc;
}
#[automatically_derived]
impl crate::Optionable for StorageVersionConditionAc {
    type Optioned = StorageVersionConditionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::apiserverinternal::v1alpha1::StorageVersionCondition {
    fn into_optioned(self) -> StorageVersionConditionAc {
        StorageVersionConditionAc {
            last_transition_time: crate::OptionableConvert::into_optioned(
                self.last_transition_time,
            ),
            message: Some(crate::OptionableConvert::into_optioned(self.message)),
            observed_generation: crate::OptionableConvert::into_optioned(
                self.observed_generation,
            ),
            reason: Some(crate::OptionableConvert::into_optioned(self.reason)),
            status: Some(crate::OptionableConvert::into_optioned(self.status)),
            type_: Some(crate::OptionableConvert::into_optioned(self.type_)),
        }
    }
    fn try_from_optioned(
        value: StorageVersionConditionAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            last_transition_time: crate::OptionableConvert::try_from_optioned(
                value.last_transition_time,
            )?,
            message: crate::OptionableConvert::try_from_optioned(
                value
                    .message
                    .ok_or(crate::Error {
                        missing_field: "message",
                    })?,
            )?,
            observed_generation: crate::OptionableConvert::try_from_optioned(
                value.observed_generation,
            )?,
            reason: crate::OptionableConvert::try_from_optioned(
                value
                    .reason
                    .ok_or(crate::Error {
                        missing_field: "reason",
                    })?,
            )?,
            status: crate::OptionableConvert::try_from_optioned(
                value
                    .status
                    .ok_or(crate::Error {
                        missing_field: "status",
                    })?,
            )?,
            type_: crate::OptionableConvert::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::Error {
                        missing_field: "type_",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: StorageVersionConditionAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.last_transition_time,
            other.last_transition_time,
        )?;
        if let Some(other_value) = other.message {
            crate::OptionableConvert::merge(&mut self.message, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.observed_generation,
            other.observed_generation,
        )?;
        if let Some(other_value) = other.reason {
            crate::OptionableConvert::merge(&mut self.reason, other_value)?;
        }
        if let Some(other_value) = other.status {
            crate::OptionableConvert::merge(&mut self.status, other_value)?;
        }
        if let Some(other_value) = other.type_ {
            crate::OptionableConvert::merge(&mut self.type_, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::apiserverinternal::v1alpha1::StorageVersionCondition,
> for StorageVersionConditionAc {
    fn from_optionable(
        value: k8s_openapi027::api::apiserverinternal::v1alpha1::StorageVersionCondition,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::apiserverinternal::v1alpha1::StorageVersionCondition,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apiserverinternal::v1alpha1::StorageVersionCondition,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
