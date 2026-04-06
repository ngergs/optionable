#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodResourceClaimStatus is stored in the PodStatus for each PodResourceClaim which references a ResourceClaimTemplate. It stores the generated name for the corresponding ResourceClaim.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodResourceClaimStatusAc {
    /// Name uniquely identifies this resource claim inside the pod. This must match the name of an entry in pod.spec.resourceClaims, which implies that the string must be a DNS_LABEL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// ResourceClaimName is the name of the ResourceClaim that was generated for the Pod in the namespace of the Pod. If this is unset, then generating a ResourceClaim was not necessary. The pod.spec.resourceClaims entry can be ignored in this case.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claim_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PodResourceClaimStatus {
    type Optioned = PodResourceClaimStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PodResourceClaimStatusAc {
    type Optioned = PodResourceClaimStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::PodResourceClaimStatus {
    fn into_optioned(self) -> PodResourceClaimStatusAc {
        PodResourceClaimStatusAc {
            name: Some(self.name),
            resource_claim_name: self.resource_claim_name,
        }
    }
    fn try_from_optioned(value: PodResourceClaimStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            resource_claim_name: value.resource_claim_name,
        })
    }
    fn merge(&mut self, other: PodResourceClaimStatusAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        self.resource_claim_name = other.resource_claim_name;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PodResourceClaimStatus>
for PodResourceClaimStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::PodResourceClaimStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PodResourceClaimStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PodResourceClaimStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
