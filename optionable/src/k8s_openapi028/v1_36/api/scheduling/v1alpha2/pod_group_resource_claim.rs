#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodGroupResourceClaim references exactly one ResourceClaim, either directly or by naming a ResourceClaimTemplate which is then turned into a ResourceClaim for the PodGroup.
///
/// It adds a name to it that uniquely identifies the ResourceClaim inside the PodGroup. Pods that need access to the ResourceClaim define a matching reference in its own Spec.ResourceClaims. The Pod's claim must match all fields of the PodGroup's claim exactly.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodGroupResourceClaimAc {
    /// Name uniquely identifies this resource claim inside the PodGroup. This must be a DNS_LABEL.
    pub name: std::string::String,
    /// ResourceClaimName is the name of a ResourceClaim object in the same namespace as this PodGroup. The ResourceClaim will be reserved for the PodGroup instead of its individual pods.
    ///
    /// Exactly one of ResourceClaimName and ResourceClaimTemplateName must be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claim_name: Option<std::string::String>,
    /// ResourceClaimTemplateName is the name of a ResourceClaimTemplate object in the same namespace as this PodGroup.
    ///
    /// The template will be used to create a new ResourceClaim, which will be bound to this PodGroup. When this PodGroup is deleted, the ResourceClaim will also be deleted. The PodGroup name and resource name, along with a generated component, will be used to form a unique name for the ResourceClaim, which will be recorded in podgroup.status.resourceClaimStatuses.
    ///
    /// This field is immutable and no changes will be made to the corresponding ResourceClaim by the control plane after creating the ResourceClaim.
    ///
    /// Exactly one of ResourceClaimName and ResourceClaimTemplateName must be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claim_template_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi028::api::scheduling::v1alpha2::PodGroupResourceClaim {
    type Optioned = PodGroupResourceClaimAc;
}
#[automatically_derived]
impl crate::Optionable for PodGroupResourceClaimAc {
    type Optioned = PodGroupResourceClaimAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi028::api::scheduling::v1alpha2::PodGroupResourceClaim {
    fn into_optioned(self) -> PodGroupResourceClaimAc {
        PodGroupResourceClaimAc {
            name: self.name,
            resource_claim_name: self.resource_claim_name,
            resource_claim_template_name: self.resource_claim_template_name,
        }
    }
    fn try_from_optioned(value: PodGroupResourceClaimAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value.name,
            resource_claim_name: value.resource_claim_name,
            resource_claim_template_name: value.resource_claim_template_name,
        })
    }
    fn merge(&mut self, other: PodGroupResourceClaimAc) -> Result<(), crate::Error> {
        self.name = other.name;
        if self.resource_claim_name.is_none() {
            self.resource_claim_name = crate::OptionableConvert::try_from_optioned(
                other.resource_claim_name,
            )?;
        } else if let Some(self_value) = self.resource_claim_name.as_mut()
            && let Some(other_value) = other.resource_claim_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.resource_claim_template_name.is_none() {
            self.resource_claim_template_name = crate::OptionableConvert::try_from_optioned(
                other.resource_claim_template_name,
            )?;
        } else if let Some(self_value) = self.resource_claim_template_name.as_mut()
            && let Some(other_value) = other.resource_claim_template_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi028::api::scheduling::v1alpha2::PodGroupResourceClaim {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.name == other.name
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi028::api::scheduling::v1alpha2::PodGroupResourceClaim,
> for PodGroupResourceClaimAc {
    fn from_optionable(
        value: k8s_openapi028::api::scheduling::v1alpha2::PodGroupResourceClaim,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi028::api::scheduling::v1alpha2::PodGroupResourceClaim,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::api::scheduling::v1alpha2::PodGroupResourceClaim,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::DeepMerge for PodGroupResourceClaimAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi028::DeepMerge::merge_from(&mut self.name, other.name);
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.resource_claim_name,
            other.resource_claim_name,
        );
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.resource_claim_template_name,
            other.resource_claim_template_name,
        );
    }
}
impl crate::merge::MapKeysEq for PodGroupResourceClaimAc {
    fn keys_eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
