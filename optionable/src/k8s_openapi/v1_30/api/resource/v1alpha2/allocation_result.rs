#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct AllocationResultAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_on_nodes: <Option<
        ::k8s_openapi::api::core::v1::NodeSelector,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_handles: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1alpha2::ResourceHandle>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shareable: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha2::AllocationResult {
    type Optioned = AllocationResultAc;
}
#[automatically_derived]
impl crate::Optionable for AllocationResultAc {
    type Optioned = AllocationResultAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha2::AllocationResult {
    fn into_optioned(self) -> AllocationResultAc {
        AllocationResultAc {
            available_on_nodes: crate::OptionableConvert::into_optioned(
                self.available_on_nodes,
            ),
            resource_handles: crate::OptionableConvert::into_optioned(
                self.resource_handles,
            ),
            shareable: crate::OptionableConvert::into_optioned(self.shareable),
        }
    }
    fn try_from_optioned(
        value: AllocationResultAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            available_on_nodes: crate::OptionableConvert::try_from_optioned(
                value.available_on_nodes,
            )?,
            resource_handles: crate::OptionableConvert::try_from_optioned(
                value.resource_handles,
            )?,
            shareable: crate::OptionableConvert::try_from_optioned(value.shareable)?,
        })
    }
    fn merge(
        &mut self,
        other: AllocationResultAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.available_on_nodes,
            other.available_on_nodes,
        )?;
        crate::OptionableConvert::merge(
            &mut self.resource_handles,
            other.resource_handles,
        )?;
        crate::OptionableConvert::merge(&mut self.shareable, other.shareable)?;
        Ok(())
    }
}
