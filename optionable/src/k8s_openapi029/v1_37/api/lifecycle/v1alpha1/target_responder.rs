#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// TargetResponder allows you to specify the responder reacting to the Eviction. Responders should observe and communicate through the Eviction API (see .state) to help with the graceful eviction of a target (e.g. termination of a pod).
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TargetResponderAc {
    /// name allows you to identify the responder reacting to the Eviction.
    ///
    /// It must be a valid domain-prefixed key (such as "acme.io/foo"). This field must be unique for each responder. This field is required.
    pub name: std::string::String,
    /// priority for this responder. Higher priorities are selected first by the evictionrequest-controller. If there are responders with the same priority, the responder whose domain name comes first in the alphabetical higher domain order, will be picked. This means that the top domain labels are compared alphabetically first, followed by the lower domain labels. The key is compared last.
    ///
    /// The responder that is the managing controller of the pod should set the value of this field to 10000 to allow both for preemption or fallback registration by other responders.
    ///
    /// The minimum value is 0 and the maximum value is 100000. The interval 0-999 is reserved for responders with *.k8s.io suffix. This field is required and immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// state specifies a state that is assigned by the evictionrequest-controller. Responders should observe this state in order to navigate their lifecycle. - Inactive means that the responder should not yet process this eviction request. - Active means that the responder is either running or expected to start soon.
    ///   Also, startTime has been set in the ResponderStatus by the evictionrequest-controller.
    ///
    ///   An active responder should currently interact with the eviction process by updating
    ///   .status.responders, where .name is the active responder name. ResponderStatus fields
    ///   should be periodically updated to indicate the progress or completion of the eviction process.
    ///   If .status.responders\[\].heartbeatTime field is not updated within the heartbeat deadline defined
    ///   by the Eviction API (currently 20 minutes), the eviction is passed over to the next responder
    ///      with a lower priority. Only one responder can be active at a time.
    /// - Interrupted means that the responder has failed to start or failed to update
    ///   heartbeatTime in ResponderStatus in a timely manner.
    /// - Canceled means that the responder has been canceled. In other words, there    is no
    ///   EvictionRequest with the same target and Eviction intent in .spec.intent.
    /// - Completed means that the responder has successfully completed and set completionTime
    ///   in ResponderStatus.
    ///
    /// Please refer to the ResponderStatus in .status.responders for more details on each responder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::lifecycle::v1alpha1::TargetResponder {
    type Optioned = TargetResponderAc;
}
#[automatically_derived]
impl crate::Optionable for TargetResponderAc {
    type Optioned = TargetResponderAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::lifecycle::v1alpha1::TargetResponder {
    fn into_optioned(self) -> TargetResponderAc {
        TargetResponderAc {
            name: self.name,
            priority: Some(self.priority),
            state: Some(self.state),
        }
    }
    fn try_from_optioned(value: TargetResponderAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value.name,
            priority: value
                .priority
                .ok_or(crate::Error {
                    missing_field: "priority",
                })?,
            state: value
                .state
                .ok_or(crate::Error {
                    missing_field: "state",
                })?,
        })
    }
    fn merge(&mut self, other: TargetResponderAc) -> Result<(), crate::Error> {
        self.name = other.name;
        if let Some(other_value) = other.priority {
            self.priority = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.state {
            self.state = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi029::api::lifecycle::v1alpha1::TargetResponder {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.name == other.name
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::lifecycle::v1alpha1::TargetResponder>
for TargetResponderAc {
    fn from_optionable(
        value: k8s_openapi029::api::lifecycle::v1alpha1::TargetResponder,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::lifecycle::v1alpha1::TargetResponder,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::lifecycle::v1alpha1::TargetResponder,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for TargetResponderAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.name, other.name);
        k8s_openapi029::DeepMerge::merge_from(&mut self.priority, other.priority);
        k8s_openapi029::DeepMerge::merge_from(&mut self.state, other.state);
    }
}
impl crate::merge::MapKeysEq for TargetResponderAc {
    fn keys_eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
