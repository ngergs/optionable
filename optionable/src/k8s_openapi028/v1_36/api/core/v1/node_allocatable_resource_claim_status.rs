#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NodeAllocatableResourceClaimStatus describes the status of node allocatable resources allocated via DRA.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeAllocatableResourceClaimStatusAc {
    /// Containers lists the names of all containers in this pod that reference the claim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<std::vec::Vec<std::string::String>>,
    /// ResourceClaimName is the resource claim referenced by the pod that resulted in this node allocatable resource allocation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claim_name: Option<std::string::String>,
    /// Resources is a map of the node-allocatable resource name to the aggregate quantity allocated to the claim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi028::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi028::api::core::v1::NodeAllocatableResourceClaimStatus {
    type Optioned = NodeAllocatableResourceClaimStatusAc;
}
#[automatically_derived]
impl crate::Optionable for NodeAllocatableResourceClaimStatusAc {
    type Optioned = NodeAllocatableResourceClaimStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi028::api::core::v1::NodeAllocatableResourceClaimStatus {
    fn into_optioned(self) -> NodeAllocatableResourceClaimStatusAc {
        NodeAllocatableResourceClaimStatusAc {
            containers: self.containers,
            resource_claim_name: Some(self.resource_claim_name),
            resources: Some(crate::OptionableConvert::into_optioned(self.resources)),
        }
    }
    fn try_from_optioned(
        value: NodeAllocatableResourceClaimStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            containers: value.containers,
            resource_claim_name: value
                .resource_claim_name
                .ok_or(crate::Error {
                    missing_field: "resource_claim_name",
                })?,
            resources: crate::OptionableConvert::try_from_optioned(
                value
                    .resources
                    .ok_or(crate::Error {
                        missing_field: "resources",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: NodeAllocatableResourceClaimStatusAc,
    ) -> Result<(), crate::Error> {
        if self.containers.is_none() {
            self.containers = crate::OptionableConvert::try_from_optioned(
                other.containers,
            )?;
        } else if let Some(self_value) = self.containers.as_mut()
            && let Some(other_value) = other.containers
        {
            crate::merge::try_merge_optioned_set(self_value, other_value)?;
        }
        if let Some(other_value) = other.resource_claim_name {
            self.resource_claim_name = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if let Some(other_value) = other.resources {
            crate::OptionableConvert::merge(&mut self.resources, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi028::api::core::v1::NodeAllocatableResourceClaimStatus,
> for NodeAllocatableResourceClaimStatusAc {
    fn from_optionable(
        value: k8s_openapi028::api::core::v1::NodeAllocatableResourceClaimStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi028::api::core::v1::NodeAllocatableResourceClaimStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::api::core::v1::NodeAllocatableResourceClaimStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::DeepMerge for NodeAllocatableResourceClaimStatusAc {
    fn merge_from(&mut self, other: Self) {
        crate::merge::merge_append_not_present_option_wrapped(
            &mut self.containers,
            other.containers,
        );
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.resource_claim_name,
            other.resource_claim_name,
        );
        crate::k8s_openapi::merge::merge_granular_option_wrapped(
            &mut self.resources,
            other.resources,
        );
    }
}
