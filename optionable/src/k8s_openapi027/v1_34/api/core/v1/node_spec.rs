#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NodeSpec describes the attributes that a node is created with.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeSpecAc {
    /// Deprecated: Previously used to specify the source of the node's configuration for the DynamicKubeletConfig feature. This feature is removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_source: Option<
        <::k8s_openapi027::api::core::v1::NodeConfigSource as crate::Optionable>::Optioned,
    >,
    /// Deprecated. Not all kubelets will set this field. Remove field after 1.13. see: https://issues.k8s.io/61966
    #[serde(rename = "externalID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<std::string::String>,
    /// PodCIDR represents the pod IP range assigned to the node.
    #[serde(rename = "podCIDR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_cidr: Option<std::string::String>,
    /// podCIDRs represents the IP ranges assigned to the node for usage by Pods on that node. If this field is specified, the 0th entry must match the podCIDR field. It may contain at most 1 value for each of IPv4 and IPv6.
    #[serde(rename = "podCIDRs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_cidrs: Option<std::vec::Vec<std::string::String>>,
    /// ID of the node assigned by the cloud provider in the format: \<ProviderName\>://\<ProviderSpecificNodeID\>
    #[serde(rename = "providerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<std::string::String>,
    /// If specified, the node's taints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::Taint as crate::Optionable>::Optioned,
        >,
    >,
    /// Unschedulable controls node schedulability of new pods. By default, node is schedulable. More info: https://kubernetes.io/docs/concepts/nodes/node/#manual-node-administration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unschedulable: Option<bool>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::NodeSpec {
    type Optioned = NodeSpecAc;
}
#[automatically_derived]
impl crate::Optionable for NodeSpecAc {
    type Optioned = NodeSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::NodeSpec {
    fn into_optioned(self) -> NodeSpecAc {
        NodeSpecAc {
            config_source: crate::OptionableConvert::into_optioned(self.config_source),
            external_id: self.external_id,
            pod_cidr: self.pod_cidr,
            pod_cidrs: self.pod_cidrs,
            provider_id: self.provider_id,
            taints: crate::OptionableConvert::into_optioned(self.taints),
            unschedulable: self.unschedulable,
        }
    }
    fn try_from_optioned(value: NodeSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            config_source: crate::OptionableConvert::try_from_optioned(
                value.config_source,
            )?,
            external_id: value.external_id,
            pod_cidr: value.pod_cidr,
            pod_cidrs: value.pod_cidrs,
            provider_id: value.provider_id,
            taints: crate::OptionableConvert::try_from_optioned(value.taints)?,
            unschedulable: value.unschedulable,
        })
    }
    fn merge(&mut self, other: NodeSpecAc) -> Result<(), crate::Error> {
        if self.config_source.is_none() {
            self.config_source = crate::OptionableConvert::try_from_optioned(
                other.config_source,
            )?;
        } else if let Some(self_value) = self.config_source.as_mut()
            && let Some(other_value) = other.config_source
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.external_id.is_none() {
            self.external_id = crate::OptionableConvert::try_from_optioned(
                other.external_id,
            )?;
        } else if let Some(self_value) = self.external_id.as_mut()
            && let Some(other_value) = other.external_id
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.pod_cidr.is_none() {
            self.pod_cidr = crate::OptionableConvert::try_from_optioned(other.pod_cidr)?;
        } else if let Some(self_value) = self.pod_cidr.as_mut()
            && let Some(other_value) = other.pod_cidr
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.pod_cidrs.is_none() {
            self.pod_cidrs = crate::OptionableConvert::try_from_optioned(
                other.pod_cidrs,
            )?;
        } else if let Some(self_value) = self.pod_cidrs.as_mut()
            && let Some(other_value) = other.pod_cidrs
        {
            crate::merge::try_merge_optioned_set(self_value, other_value)?;
        }
        if self.provider_id.is_none() {
            self.provider_id = crate::OptionableConvert::try_from_optioned(
                other.provider_id,
            )?;
        } else if let Some(self_value) = self.provider_id.as_mut()
            && let Some(other_value) = other.provider_id
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.taints.is_none() {
            self.taints = crate::OptionableConvert::try_from_optioned(other.taints)?;
        } else if let Some(self_value) = self.taints.as_mut()
            && let Some(other_value) = other.taints
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.unschedulable.is_none() {
            self.unschedulable = crate::OptionableConvert::try_from_optioned(
                other.unschedulable,
            )?;
        } else if let Some(self_value) = self.unschedulable.as_mut()
            && let Some(other_value) = other.unschedulable
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::NodeSpec> for NodeSpecAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::NodeSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::NodeSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::NodeSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for NodeSpecAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.config_source,
            other.config_source,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.external_id, other.external_id);
        k8s_openapi027::DeepMerge::merge_from(&mut self.pod_cidr, other.pod_cidr);
        crate::merge::merge_append_not_present(&mut self.pod_cidrs, other.pod_cidrs);
        k8s_openapi027::DeepMerge::merge_from(&mut self.provider_id, other.provider_id);
        self.taints = other.taints;
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.unschedulable,
            other.unschedulable,
        );
    }
}
