#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodSchedulingContextSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub potential_nodes: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_node: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::resource::v1alpha3::PodSchedulingContextSpec {
    type Optioned = PodSchedulingContextSpecAc;
}
#[automatically_derived]
impl crate::Optionable for PodSchedulingContextSpecAc {
    type Optioned = PodSchedulingContextSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1alpha3::PodSchedulingContextSpec {
    fn into_optioned(self) -> PodSchedulingContextSpecAc {
        PodSchedulingContextSpecAc {
            potential_nodes: self.potential_nodes,
            selected_node: self.selected_node,
        }
    }
    fn try_from_optioned(
        value: PodSchedulingContextSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            potential_nodes: value.potential_nodes,
            selected_node: value.selected_node,
        })
    }
    fn merge(&mut self, other: PodSchedulingContextSpecAc) -> Result<(), crate::Error> {
        self.potential_nodes = other.potential_nodes;
        self.selected_node = other.selected_node;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::resource::v1alpha3::PodSchedulingContextSpec,
> for PodSchedulingContextSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::PodSchedulingContextSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1alpha3::PodSchedulingContextSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::PodSchedulingContextSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
