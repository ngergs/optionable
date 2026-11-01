#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NodeAllocatableMappedResources describes mapped node allocatable resource allocations.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeAllocatableMappedResourcesAc {
    /// Name is the name of the resource (e.g., cpu, memory).
    pub name: std::string::String,
    /// Quantity is the total node allocatable resource capacity allocated for the claim. This claim's allocated devices is shared by all the containers referencing the claim. Kubelet adds this value to both requests and limits at the pod-level cgroup, and to limits at the container-level cgroup for each container referencing the claim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<
        <::k8s_openapi029::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::core::v1::NodeAllocatableMappedResources {
    type Optioned = NodeAllocatableMappedResourcesAc;
}
#[automatically_derived]
impl crate::Optionable for NodeAllocatableMappedResourcesAc {
    type Optioned = NodeAllocatableMappedResourcesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::core::v1::NodeAllocatableMappedResources {
    fn into_optioned(self) -> NodeAllocatableMappedResourcesAc {
        NodeAllocatableMappedResourcesAc {
            name: self.name,
            quantity: Some(crate::OptionableConvert::into_optioned(self.quantity)),
        }
    }
    fn try_from_optioned(
        value: NodeAllocatableMappedResourcesAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value.name,
            quantity: crate::OptionableConvert::try_from_optioned(
                value
                    .quantity
                    .ok_or(crate::Error {
                        missing_field: "quantity",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: NodeAllocatableMappedResourcesAc,
    ) -> Result<(), crate::Error> {
        self.name = other.name;
        if let Some(other_value) = other.quantity {
            crate::OptionableConvert::merge(&mut self.quantity, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi029::api::core::v1::NodeAllocatableMappedResources {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.name == other.name
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi029::api::core::v1::NodeAllocatableMappedResources,
> for NodeAllocatableMappedResourcesAc {
    fn from_optionable(
        value: k8s_openapi029::api::core::v1::NodeAllocatableMappedResources,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::core::v1::NodeAllocatableMappedResources,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::core::v1::NodeAllocatableMappedResources,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for NodeAllocatableMappedResourcesAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.name, other.name);
        k8s_openapi029::DeepMerge::merge_from(&mut self.quantity, other.quantity);
    }
}
impl crate::merge::MapKeysEq for NodeAllocatableMappedResourcesAc {
    fn keys_eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
