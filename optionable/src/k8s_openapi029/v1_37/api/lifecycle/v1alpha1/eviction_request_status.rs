#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// EvictionRequestStatus represents the last observed status of the eviction request.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvictionRequestStatusAc {
    /// conditions contain information about the eviction request.
    ///
    /// EvictionRequest specific conditions are: TargetEvicted or Failed (managed by evictionrequest-controller). - Failed means that the eviction request is no longer being processed
    ///   by any eviction responder. This can happen if the request is canceled or if no responder
    ///   managed to evict the target (e.g. terminate or delete a pod).
    /// - TargetEvicted means that the target has been evicted (e.g. a pod has been terminated or deleted).
    ///
    /// These conditions can be reset if the eviction was unsuccessful and a new Eviction intent has been submitted.
    ///
    /// The maximum length of the conditions list is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi029::apimachinery::pkg::apis::meta::v1::Condition as crate::Optionable>::Optioned,
        >,
    >,
    /// observedGeneration is EvictionRequest's .metadata.generation observed by the evictionrequest-controller. The observed generation value cannot be negative and can only be incremented. The minimum value is 1. This field is managed by evictionrequest-controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequestStatus {
    type Optioned = EvictionRequestStatusAc;
}
#[automatically_derived]
impl crate::Optionable for EvictionRequestStatusAc {
    type Optioned = EvictionRequestStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequestStatus {
    fn into_optioned(self) -> EvictionRequestStatusAc {
        EvictionRequestStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            observed_generation: self.observed_generation,
        }
    }
    fn try_from_optioned(value: EvictionRequestStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            observed_generation: value.observed_generation,
        })
    }
    fn merge(&mut self, other: EvictionRequestStatusAc) -> Result<(), crate::Error> {
        if self.conditions.is_none() {
            self.conditions = crate::OptionableConvert::try_from_optioned(
                other.conditions,
            )?;
        } else if let Some(self_value) = self.conditions.as_mut()
            && let Some(other_value) = other.conditions
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if self.observed_generation.is_none() {
            self.observed_generation = crate::OptionableConvert::try_from_optioned(
                other.observed_generation,
            )?;
        } else if let Some(self_value) = self.observed_generation.as_mut()
            && let Some(other_value) = other.observed_generation
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequestStatus,
> for EvictionRequestStatusAc {
    fn from_optionable(
        value: k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequestStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequestStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::lifecycle::v1alpha1::EvictionRequestStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for EvictionRequestStatusAc {
    fn merge_from(&mut self, other: Self) {
        crate::k8s_openapi::merge::merge_map_option_wrapped(
            &mut self.conditions,
            other.conditions,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.observed_generation,
            other.observed_generation,
        );
    }
}
