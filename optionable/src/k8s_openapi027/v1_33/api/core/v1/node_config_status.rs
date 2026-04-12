#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NodeConfigStatus describes the status of the config assigned by Node.Spec.ConfigSource.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeConfigStatusAc {
    /// Active reports the checkpointed config the node is actively using. Active will represent either the current version of the Assigned config, or the current LastKnownGood config, depending on whether attempting to use the Assigned config results in an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<
        <::k8s_openapi027::api::core::v1::NodeConfigSource as crate::Optionable>::Optioned,
    >,
    /// Assigned reports the checkpointed config the node will try to use. When Node.Spec.ConfigSource is updated, the node checkpoints the associated config payload to local disk, along with a record indicating intended config. The node refers to this record to choose its config checkpoint, and reports this record in Assigned. Assigned only updates in the status after the record has been checkpointed to disk. When the Kubelet is restarted, it tries to make the Assigned config the Active config by loading and validating the checkpointed payload identified by Assigned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned: Option<
        <::k8s_openapi027::api::core::v1::NodeConfigSource as crate::Optionable>::Optioned,
    >,
    /// Error describes any problems reconciling the Spec.ConfigSource to the Active config. Errors may occur, for example, attempting to checkpoint Spec.ConfigSource to the local Assigned record, attempting to checkpoint the payload associated with Spec.ConfigSource, attempting to load or validate the Assigned config, etc. Errors may occur at different points while syncing config. Earlier errors (e.g. download or checkpointing errors) will not result in a rollback to LastKnownGood, and may resolve across Kubelet retries. Later errors (e.g. loading or validating a checkpointed config) will result in a rollback to LastKnownGood. In the latter case, it is usually possible to resolve the error by fixing the config assigned in Spec.ConfigSource. You can find additional information for debugging by searching the error message in the Kubelet log. Error is a human-readable description of the error state; machines can check whether or not Error is empty, but should not rely on the stability of the Error text across Kubelet versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<std::string::String>,
    /// LastKnownGood reports the checkpointed config the node will fall back to when it encounters an error attempting to use the Assigned config. The Assigned config becomes the LastKnownGood config when the node determines that the Assigned config is stable and correct. This is currently implemented as a 10-minute soak period starting when the local record of Assigned config is updated. If the Assigned config is Active at the end of this period, it becomes the LastKnownGood. Note that if Spec.ConfigSource is reset to nil (use local defaults), the LastKnownGood is also immediately reset to nil, because the local default config is always assumed good. You should not make assumptions about the node's method of determining config stability and correctness, as this may change or become configurable in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_known_good: Option<
        <::k8s_openapi027::api::core::v1::NodeConfigSource as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::NodeConfigStatus {
    type Optioned = NodeConfigStatusAc;
}
#[automatically_derived]
impl crate::Optionable for NodeConfigStatusAc {
    type Optioned = NodeConfigStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::NodeConfigStatus {
    fn into_optioned(self) -> NodeConfigStatusAc {
        NodeConfigStatusAc {
            active: crate::OptionableConvert::into_optioned(self.active),
            assigned: crate::OptionableConvert::into_optioned(self.assigned),
            error: self.error,
            last_known_good: crate::OptionableConvert::into_optioned(
                self.last_known_good,
            ),
        }
    }
    fn try_from_optioned(value: NodeConfigStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            active: crate::OptionableConvert::try_from_optioned(value.active)?,
            assigned: crate::OptionableConvert::try_from_optioned(value.assigned)?,
            error: value.error,
            last_known_good: crate::OptionableConvert::try_from_optioned(
                value.last_known_good,
            )?,
        })
    }
    fn merge(&mut self, other: NodeConfigStatusAc) -> Result<(), crate::Error> {
        if self.active.is_none() {
            self.active = other.active;
        }
        if let Some(other_value) = other.active {
            crate::OptionableConvert::merge(&mut self.active, other_value)?;
        }
        if self.assigned.is_none() {
            self.assigned = other.assigned;
        }
        if let Some(other_value) = other.assigned {
            crate::OptionableConvert::merge(&mut self.assigned, other_value)?;
        }
        if self.error.is_none() {
            self.error = other.error;
        }
        if let Some(other_value) = other.error {
            crate::OptionableConvert::merge(&mut self.error, other_value)?;
        }
        if self.last_known_good.is_none() {
            self.last_known_good = other.last_known_good;
        }
        if let Some(other_value) = other.last_known_good {
            crate::OptionableConvert::merge(&mut self.last_known_good, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::NodeConfigStatus>
for NodeConfigStatusAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::NodeConfigStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::NodeConfigStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::NodeConfigStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
