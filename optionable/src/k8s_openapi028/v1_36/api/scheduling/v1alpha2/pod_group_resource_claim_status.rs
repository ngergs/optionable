#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodGroupResourceClaimStatus is stored in the PodGroupStatus for each PodGroupResourceClaim which references a ResourceClaimTemplate. It stores the generated name for the corresponding ResourceClaim.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodGroupResourceClaimStatusAc {
    /// Name uniquely identifies this resource claim inside the PodGroup. This must match the name of an entry in podgroup.spec.resourceClaims, which implies that the string must be a DNS_LABEL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// ResourceClaimName is the name of the ResourceClaim that was generated for the PodGroup in the namespace of the PodGroup. If this is unset, then generating a ResourceClaim was not necessary. The podgroup.spec.resourceClaims entry can be ignored in this case.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claim_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi028::api::scheduling::v1alpha2::PodGroupResourceClaimStatus {
    type Optioned = PodGroupResourceClaimStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PodGroupResourceClaimStatusAc {
    type Optioned = PodGroupResourceClaimStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi028::api::scheduling::v1alpha2::PodGroupResourceClaimStatus {
    fn into_optioned(self) -> PodGroupResourceClaimStatusAc {
        PodGroupResourceClaimStatusAc {
            name: Some(self.name),
            resource_claim_name: self.resource_claim_name,
        }
    }
    fn try_from_optioned(
        value: PodGroupResourceClaimStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            resource_claim_name: value.resource_claim_name,
        })
    }
    fn merge(
        &mut self,
        other: PodGroupResourceClaimStatusAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.resource_claim_name.is_none() {
            self.resource_claim_name = crate::OptionableConvert::try_from_optioned(
                other.resource_claim_name,
            )?;
        } else if let Some(self_value) = self.resource_claim_name.as_mut()
            && let Some(other_value) = other.resource_claim_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi028::api::scheduling::v1alpha2::PodGroupResourceClaimStatus,
> for PodGroupResourceClaimStatusAc {
    fn from_optionable(
        value: k8s_openapi028::api::scheduling::v1alpha2::PodGroupResourceClaimStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi028::api::scheduling::v1alpha2::PodGroupResourceClaimStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::api::scheduling::v1alpha2::PodGroupResourceClaimStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::DeepMerge for PodGroupResourceClaimStatusAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi028::DeepMerge::merge_from(&mut self.name, other.name);
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.resource_claim_name,
            other.resource_claim_name,
        );
    }
}
