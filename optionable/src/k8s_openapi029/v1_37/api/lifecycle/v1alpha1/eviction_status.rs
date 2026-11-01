#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// EvictionStatus represents the last observed status of the eviction request.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvictionStatusAc {
    /// conditions contain information about the eviction request.
    ///
    /// Eviction specific conditions are: TargetEvicted or Failed (managed by evictionrequest-controller). - Failed means that the eviction request is no longer being processed
    ///   by any eviction responder. This can happen if the request is canceled or if no responder
    ///   managed to evict the target (e.g. terminate or delete a pod).
    /// - TargetEvicted means that the target has been evicted (e.g. a pod has been terminated or deleted).
    ///
    ///   The maximum length of the conditions list is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi029::apimachinery::pkg::apis::meta::v1::Condition as crate::Optionable>::Optioned,
        >,
    >,
    /// observedGeneration is Eviction's .metadata.generation observed by the evictionrequest-controller. The observed generation value cannot be negative and can only be incremented. The minimum value is 1. This field is managed by evictionrequest-controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// requesters allow you to identify the entities, that requested the eviction of the target. If all the requesters withdraw their eviction intent, the eviction will be canceled.
    ///
    /// The maximum length of the requesters list is 100. If this limit is exceeded, requesters with Withdrawn intent should be dropped first.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requesters: Option<
        std::vec::Vec<
            <::k8s_openapi029::api::lifecycle::v1alpha1::Requester as crate::Optionable>::Optioned,
        >,
    >,
    /// responders represents the eviction process status of each declared responder.
    ///
    /// The responder list should be the same length and have the same .name fields as .status.targetResponders. Only responders with .name that have Active state in .targetResponders\[\].state should be updated and can be mutated. First initialization of the list is allowed.
    ///
    /// Each ResponderStatus is initialized by evictionrequest-controller and then managed by the designated responder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responders: Option<
        std::vec::Vec<
            <::k8s_openapi029::api::lifecycle::v1alpha1::ResponderStatus as crate::Optionable>::Optioned,
        >,
    >,
    /// targetResponders reference responders that should eventually respond to this eviction to help with the graceful eviction of a target. These responders are selected sequentially, according to their specified priority by setting the Active state to the TargetResponder .state field. The maximum number of active responders allowed is 1. Eventually each responder can end up in an Interrupted, Canceled or, Completed state. Responders should observe these states in order to navigate their lifecycle.
    ///
    /// If the target is a pod, the field is populated from Pod's .spec.evictionResponders. Default responders may be added to the list according to the target.
    ///
    /// Default responders: - imperative-eviction.k8s.io/evictor responder with a priority of 100 is added to the list if the
    ///   target is a pod. It will call the imperative Eviction API (pods/\<name\>/eviction subresource).
    ///   This call may not succeed due to PodDisruptionBudgets, which may block the pod termination.
    ///   It will update the responder message and try again with a backoff.
    ///
    /// The maximum length of the responders list is 11. The length and keys of the list cannot change once set. This field is managed by evictionrequest-controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_responders: Option<
        std::vec::Vec<
            <::k8s_openapi029::api::lifecycle::v1alpha1::TargetResponder as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::lifecycle::v1alpha1::EvictionStatus {
    type Optioned = EvictionStatusAc;
}
#[automatically_derived]
impl crate::Optionable for EvictionStatusAc {
    type Optioned = EvictionStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::lifecycle::v1alpha1::EvictionStatus {
    fn into_optioned(self) -> EvictionStatusAc {
        EvictionStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            observed_generation: self.observed_generation,
            requesters: crate::OptionableConvert::into_optioned(self.requesters),
            responders: crate::OptionableConvert::into_optioned(self.responders),
            target_responders: crate::OptionableConvert::into_optioned(
                self.target_responders,
            ),
        }
    }
    fn try_from_optioned(value: EvictionStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            observed_generation: value.observed_generation,
            requesters: crate::OptionableConvert::try_from_optioned(value.requesters)?,
            responders: crate::OptionableConvert::try_from_optioned(value.responders)?,
            target_responders: crate::OptionableConvert::try_from_optioned(
                value.target_responders,
            )?,
        })
    }
    fn merge(&mut self, other: EvictionStatusAc) -> Result<(), crate::Error> {
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
        if self.requesters.is_none() {
            self.requesters = crate::OptionableConvert::try_from_optioned(
                other.requesters,
            )?;
        } else if let Some(self_value) = self.requesters.as_mut()
            && let Some(other_value) = other.requesters
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if self.responders.is_none() {
            self.responders = crate::OptionableConvert::try_from_optioned(
                other.responders,
            )?;
        } else if let Some(self_value) = self.responders.as_mut()
            && let Some(other_value) = other.responders
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if self.target_responders.is_none() {
            self.target_responders = crate::OptionableConvert::try_from_optioned(
                other.target_responders,
            )?;
        } else if let Some(self_value) = self.target_responders.as_mut()
            && let Some(other_value) = other.target_responders
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::lifecycle::v1alpha1::EvictionStatus>
for EvictionStatusAc {
    fn from_optionable(
        value: k8s_openapi029::api::lifecycle::v1alpha1::EvictionStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi029::api::lifecycle::v1alpha1::EvictionStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::lifecycle::v1alpha1::EvictionStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for EvictionStatusAc {
    fn merge_from(&mut self, other: Self) {
        crate::k8s_openapi::merge::merge_map_option_wrapped(
            &mut self.conditions,
            other.conditions,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.observed_generation,
            other.observed_generation,
        );
        crate::k8s_openapi::merge::merge_map_option_wrapped(
            &mut self.requesters,
            other.requesters,
        );
        crate::k8s_openapi::merge::merge_map_option_wrapped(
            &mut self.responders,
            other.responders,
        );
        crate::k8s_openapi::merge::merge_map_option_wrapped(
            &mut self.target_responders,
            other.target_responders,
        );
    }
}
