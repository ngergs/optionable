#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodResourceClaim references exactly one ResourceClaim, either directly or by naming a ResourceClaimTemplate which is then turned into a ResourceClaim for the pod.
///
/// It adds a name to it that uniquely identifies the ResourceClaim inside the Pod. Containers that need access to the ResourceClaim reference it with this name.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodResourceClaimAc {
    /// Name uniquely identifies this resource claim inside the pod. This must be a DNS_LABEL.
    pub name: std::string::String,
    /// ResourceClaimName is the name of a ResourceClaim object in the same namespace as this pod.
    ///
    /// Exactly one of ResourceClaimName and ResourceClaimTemplateName must be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claim_name: Option<std::string::String>,
    /// ResourceClaimTemplateName is the name of a ResourceClaimTemplate object in the same namespace as this pod.
    ///
    /// The template will be used to create a new ResourceClaim, which will be bound to this pod. When this pod is deleted, the ResourceClaim will also be deleted. The pod name and resource name, along with a generated component, will be used to form a unique name for the ResourceClaim, which will be recorded in pod.status.resourceClaimStatuses.
    ///
    /// This field is immutable and no changes will be made to the corresponding ResourceClaim by the control plane after creating the ResourceClaim.
    ///
    /// Exactly one of ResourceClaimName and ResourceClaimTemplateName must be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claim_template_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PodResourceClaim {
    type Optioned = PodResourceClaimAc;
}
#[automatically_derived]
impl crate::Optionable for PodResourceClaimAc {
    type Optioned = PodResourceClaimAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::PodResourceClaim {
    fn into_optioned(self) -> PodResourceClaimAc {
        PodResourceClaimAc {
            name: self.name,
            resource_claim_name: self.resource_claim_name,
            resource_claim_template_name: self.resource_claim_template_name,
        }
    }
    fn try_from_optioned(value: PodResourceClaimAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value.name,
            resource_claim_name: value.resource_claim_name,
            resource_claim_template_name: value.resource_claim_template_name,
        })
    }
    fn merge(&mut self, other: PodResourceClaimAc) -> Result<(), crate::Error> {
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
for k8s_openapi027::api::core::v1::PodResourceClaim {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.name == other.name
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PodResourceClaim>
for PodResourceClaimAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::PodResourceClaim) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PodResourceClaim, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PodResourceClaim,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
