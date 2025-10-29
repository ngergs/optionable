pub struct NodeSpecOpt {
    pub config_source: <Option<
        ::k8s_openapi::api::core::v1::NodeConfigSource,
    > as crate::Optionable>::Optioned,
    pub external_id: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub pod_cidr: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub pod_cidrs: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub provider_id: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub taints: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::Taint>,
    > as crate::Optionable>::Optioned,
    pub unschedulable: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NodeSpec {
    type Optioned = NodeSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for NodeSpecOpt {
    type Optioned = NodeSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NodeSpec {
    fn into_optioned(self) -> NodeSpecOpt {
        NodeSpecOpt {
            config_source: crate::OptionableConvert::into_optioned(self.config_source),
            external_id: crate::OptionableConvert::into_optioned(self.external_id),
            pod_cidr: crate::OptionableConvert::into_optioned(self.pod_cidr),
            pod_cidrs: crate::OptionableConvert::into_optioned(self.pod_cidrs),
            provider_id: crate::OptionableConvert::into_optioned(self.provider_id),
            taints: crate::OptionableConvert::into_optioned(self.taints),
            unschedulable: crate::OptionableConvert::into_optioned(self.unschedulable),
        }
    }
    fn try_from_optioned(value: NodeSpecOpt) -> Result<Self, crate::optionable::Error> {
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
    fn merge(&mut self, other: NodeSpecOpt) -> Result<(), crate::optionable::Error> {
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
