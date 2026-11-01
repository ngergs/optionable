#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ResponderStatus represents the last observed status of the eviction process of the responder. It should be only updated by the designated responder whose name is .name field.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResponderStatusAc {
    /// completionTime tracks the time at which the Responder stopped processing the eviction request. Completion means that the responders has either fully or partially completed the eviction process, which may have resulted in target eviction (e.g. pod termination). It should reflect the present time when set. This field becomes immutable once set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<
        <::k8s_openapi029::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// expectedCompletionTime is the time at which the eviction process step is expected to end for the responder. The time cannot be set to the past. May be omitted if no estimate can be made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_completion_time: Option<
        <::k8s_openapi029::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// heartbeatTime is the last time at which the eviction process was reported to be in progress by the responder. It should reflect the present time when set. Responders should avoid heartbeats more frequent than 20 seconds to avoid overloading the control-plane.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_time: Option<
        <::k8s_openapi029::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// message provides human-readable details about the state of the responder and the eviction process. Maximum length is 4000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// name allows you to identify the responder reacting to the Eviction.
    ///
    /// It must be a valid domain-prefixed key (such as "acme.io/foo"). This field is initialized by Kubernetes and must be unique for each responder. This field is required.
    pub name: std::string::String,
    /// startTime tracks the time at which this responder was designated as active and should start processing the eviction request. It should reflect the present time when set. This field is initialized by Kubernetes when this responder becomes active. This field becomes immutable once set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<
        <::k8s_openapi029::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::lifecycle::v1alpha1::ResponderStatus {
    type Optioned = ResponderStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ResponderStatusAc {
    type Optioned = ResponderStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::lifecycle::v1alpha1::ResponderStatus {
    fn into_optioned(self) -> ResponderStatusAc {
        ResponderStatusAc {
            completion_time: crate::OptionableConvert::into_optioned(
                self.completion_time,
            ),
            expected_completion_time: crate::OptionableConvert::into_optioned(
                self.expected_completion_time,
            ),
            heartbeat_time: crate::OptionableConvert::into_optioned(self.heartbeat_time),
            message: self.message,
            name: self.name,
            start_time: crate::OptionableConvert::into_optioned(self.start_time),
        }
    }
    fn try_from_optioned(value: ResponderStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            completion_time: crate::OptionableConvert::try_from_optioned(
                value.completion_time,
            )?,
            expected_completion_time: crate::OptionableConvert::try_from_optioned(
                value.expected_completion_time,
            )?,
            heartbeat_time: crate::OptionableConvert::try_from_optioned(
                value.heartbeat_time,
            )?,
            message: value.message,
            name: value.name,
            start_time: crate::OptionableConvert::try_from_optioned(value.start_time)?,
        })
    }
    fn merge(&mut self, other: ResponderStatusAc) -> Result<(), crate::Error> {
        if self.completion_time.is_none() {
            self.completion_time = crate::OptionableConvert::try_from_optioned(
                other.completion_time,
            )?;
        } else if let Some(self_value) = self.completion_time.as_mut()
            && let Some(other_value) = other.completion_time
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.expected_completion_time.is_none() {
            self.expected_completion_time = crate::OptionableConvert::try_from_optioned(
                other.expected_completion_time,
            )?;
        } else if let Some(self_value) = self.expected_completion_time.as_mut()
            && let Some(other_value) = other.expected_completion_time
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.heartbeat_time.is_none() {
            self.heartbeat_time = crate::OptionableConvert::try_from_optioned(
                other.heartbeat_time,
            )?;
        } else if let Some(self_value) = self.heartbeat_time.as_mut()
            && let Some(other_value) = other.heartbeat_time
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.message.is_none() {
            self.message = crate::OptionableConvert::try_from_optioned(other.message)?;
        } else if let Some(self_value) = self.message.as_mut()
            && let Some(other_value) = other.message
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        self.name = other.name;
        if self.start_time.is_none() {
            self.start_time = crate::OptionableConvert::try_from_optioned(
                other.start_time,
            )?;
        } else if let Some(self_value) = self.start_time.as_mut()
            && let Some(other_value) = other.start_time
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi029::api::lifecycle::v1alpha1::ResponderStatus {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.name == other.name
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::lifecycle::v1alpha1::ResponderStatus>
for ResponderStatusAc {
    fn from_optionable(
        value: k8s_openapi029::api::lifecycle::v1alpha1::ResponderStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::lifecycle::v1alpha1::ResponderStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::lifecycle::v1alpha1::ResponderStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for ResponderStatusAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.completion_time,
            other.completion_time,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.expected_completion_time,
            other.expected_completion_time,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.heartbeat_time,
            other.heartbeat_time,
        );
        k8s_openapi029::DeepMerge::merge_from(&mut self.message, other.message);
        k8s_openapi029::DeepMerge::merge_from(&mut self.name, other.name);
        k8s_openapi029::DeepMerge::merge_from(&mut self.start_time, other.start_time);
    }
}
impl crate::merge::MapKeysEq for ResponderStatusAc {
    fn keys_eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
