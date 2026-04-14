#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Scheduling specifies the scheduling constraints for nodes supporting a RuntimeClass.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SchedulingAc {
    /// nodeSelector lists labels that must be present on nodes that support this RuntimeClass. Pods using this RuntimeClass can only be scheduled to a node matched by this selector. The RuntimeClass nodeSelector is merged with a pod's existing nodeSelector. Any conflicts will cause the pod to be rejected in admission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
    /// tolerations are appended (excluding duplicates) to pods running with this RuntimeClass during admission, effectively unioning the set of nodes tolerated by the pod and the RuntimeClass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::Toleration as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::node::v1::Scheduling {
    type Optioned = SchedulingAc;
}
#[automatically_derived]
impl crate::Optionable for SchedulingAc {
    type Optioned = SchedulingAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::node::v1::Scheduling {
    fn into_optioned(self) -> SchedulingAc {
        SchedulingAc {
            node_selector: self.node_selector,
            tolerations: crate::OptionableConvert::into_optioned(self.tolerations),
        }
    }
    fn try_from_optioned(value: SchedulingAc) -> Result<Self, crate::Error> {
        Ok(Self {
            node_selector: value.node_selector,
            tolerations: crate::OptionableConvert::try_from_optioned(value.tolerations)?,
        })
    }
    fn merge(&mut self, other: SchedulingAc) -> Result<(), crate::Error> {
        if self.node_selector.is_none() {
            self.node_selector = crate::OptionableConvert::try_from_optioned(
                other.node_selector,
            )?;
        } else if let Some(self_value) = self.node_selector.as_mut()
            && let Some(other_value) = other.node_selector
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.tolerations.is_none() {
            self.tolerations = crate::OptionableConvert::try_from_optioned(
                other.tolerations,
            )?;
        } else if let Some(self_value) = self.tolerations.as_mut()
            && let Some(other_value) = other.tolerations
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::node::v1::Scheduling> for SchedulingAc {
    fn from_optionable(value: k8s_openapi027::api::node::v1::Scheduling) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::node::v1::Scheduling, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::node::v1::Scheduling,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
