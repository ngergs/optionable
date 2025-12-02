#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct NodeSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_source: <Option<
        ::k8s_openapi::api::core::v1::NodeConfigSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_cidr: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(rename = "podCIDRs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_cidrs: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::Taint>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unschedulable: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NodeSpec {
    type Optioned = NodeSpecAc;
}
#[automatically_derived]
impl crate::Optionable for NodeSpecAc {
    type Optioned = NodeSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NodeSpec {
    fn into_optioned(self) -> NodeSpecAc {
        NodeSpecAc {
            config_source: crate::OptionableConvert::into_optioned(self.config_source),
            external_id: crate::OptionableConvert::into_optioned(self.external_id),
            pod_cidr: crate::OptionableConvert::into_optioned(self.pod_cidr),
            pod_cidrs: crate::OptionableConvert::into_optioned(self.pod_cidrs),
            provider_id: crate::OptionableConvert::into_optioned(self.provider_id),
            taints: crate::OptionableConvert::into_optioned(self.taints),
            unschedulable: crate::OptionableConvert::into_optioned(self.unschedulable),
        }
    }
    fn try_from_optioned(value: NodeSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            config_source: crate::OptionableConvert::try_from_optioned(
                value.config_source,
            )?,
            external_id: crate::OptionableConvert::try_from_optioned(value.external_id)?,
            pod_cidr: crate::OptionableConvert::try_from_optioned(value.pod_cidr)?,
            pod_cidrs: crate::OptionableConvert::try_from_optioned(value.pod_cidrs)?,
            provider_id: crate::OptionableConvert::try_from_optioned(value.provider_id)?,
            taints: crate::OptionableConvert::try_from_optioned(value.taints)?,
            unschedulable: crate::OptionableConvert::try_from_optioned(
                value.unschedulable,
            )?,
        })
    }
    fn merge(&mut self, other: NodeSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.config_source, other.config_source)?;
        crate::OptionableConvert::merge(&mut self.external_id, other.external_id)?;
        crate::OptionableConvert::merge(&mut self.pod_cidr, other.pod_cidr)?;
        crate::OptionableConvert::merge(&mut self.pod_cidrs, other.pod_cidrs)?;
        crate::OptionableConvert::merge(&mut self.provider_id, other.provider_id)?;
        crate::OptionableConvert::merge(&mut self.taints, other.taints)?;
        crate::OptionableConvert::merge(&mut self.unschedulable, other.unschedulable)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::NodeSpec> for NodeSpecAc {
    fn from_optionable(value: ::k8s_openapi::api::core::v1::NodeSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::NodeSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::NodeSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
