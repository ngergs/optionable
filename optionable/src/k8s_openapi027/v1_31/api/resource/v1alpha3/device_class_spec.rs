#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DeviceClassSpec is used in a \[DeviceClass\] to define what can be allocated and how to configure it.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceClassSpecAc {
    /// Config defines configuration parameters that apply to each device that is claimed via this class. Some classses may potentially be satisfied by multiple drivers, so each instance of a vendor configuration applies to exactly one driver.
    ///
    /// They are passed to the driver, but are not considered while allocating the claim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1alpha3::DeviceClassConfiguration as crate::Optionable>::Optioned,
        >,
    >,
    /// Each selector must be satisfied by a device which is claimed via this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1alpha3::DeviceSelector as crate::Optionable>::Optioned,
        >,
    >,
    /// Only nodes matching the selector will be considered by the scheduler when trying to find a Node that fits a Pod when that Pod uses a claim that has not been allocated yet *and* that claim gets allocated through a control plane controller. It is ignored when the claim does not use a control plane controller for allocation.
    ///
    /// Setting this field is optional. If unset, all Nodes are candidates.
    ///
    /// This is an alpha field and requires enabling the DRAControlPlaneController feature gate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suitable_nodes: Option<
        <::k8s_openapi027::api::core::v1::NodeSelector as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1alpha3::DeviceClassSpec {
    type Optioned = DeviceClassSpecAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceClassSpecAc {
    type Optioned = DeviceClassSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1alpha3::DeviceClassSpec {
    fn into_optioned(self) -> DeviceClassSpecAc {
        DeviceClassSpecAc {
            config: crate::OptionableConvert::into_optioned(self.config),
            selectors: crate::OptionableConvert::into_optioned(self.selectors),
            suitable_nodes: crate::OptionableConvert::into_optioned(self.suitable_nodes),
        }
    }
    fn try_from_optioned(value: DeviceClassSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            config: crate::OptionableConvert::try_from_optioned(value.config)?,
            selectors: crate::OptionableConvert::try_from_optioned(value.selectors)?,
            suitable_nodes: crate::OptionableConvert::try_from_optioned(
                value.suitable_nodes,
            )?,
        })
    }
    fn merge(&mut self, other: DeviceClassSpecAc) -> Result<(), crate::Error> {
        if self.config.is_none() {
            self.config = other.config;
        }
        if let Some(other_value) = other.config {
            crate::OptionableConvert::merge(&mut self.config, other_value)?;
        }
        if self.selectors.is_none() {
            self.selectors = other.selectors;
        }
        if let Some(other_value) = other.selectors {
            crate::OptionableConvert::merge(&mut self.selectors, other_value)?;
        }
        if self.suitable_nodes.is_none() {
            self.suitable_nodes = other.suitable_nodes;
        }
        if let Some(other_value) = other.suitable_nodes {
            crate::OptionableConvert::merge(&mut self.suitable_nodes, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1alpha3::DeviceClassSpec>
for DeviceClassSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::DeviceClassSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1alpha3::DeviceClassSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::DeviceClassSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
