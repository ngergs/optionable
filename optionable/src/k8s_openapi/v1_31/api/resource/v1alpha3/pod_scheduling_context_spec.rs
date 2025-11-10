#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct PodSchedulingContextSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub potential_nodes: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_node: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1alpha3::PodSchedulingContextSpec {
    type Optioned = PodSchedulingContextSpecAc;
}
#[automatically_derived]
impl crate::Optionable for PodSchedulingContextSpecAc {
    type Optioned = PodSchedulingContextSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha3::PodSchedulingContextSpec {
    fn into_optioned(self) -> PodSchedulingContextSpecAc {
        PodSchedulingContextSpecAc {
            potential_nodes: crate::OptionableConvert::into_optioned(
                self.potential_nodes,
            ),
            selected_node: crate::OptionableConvert::into_optioned(self.selected_node),
        }
    }
    fn try_from_optioned(
        value: PodSchedulingContextSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            potential_nodes: crate::OptionableConvert::try_from_optioned(
                value.potential_nodes,
            )?,
            selected_node: crate::OptionableConvert::try_from_optioned(
                value.selected_node,
            )?,
        })
    }
    fn merge(&mut self, other: PodSchedulingContextSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.potential_nodes,
            other.potential_nodes,
        )?;
        crate::OptionableConvert::merge(&mut self.selected_node, other.selected_node)?;
        Ok(())
    }
}
