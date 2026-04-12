#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodExtendedResourceClaimStatus is stored in the PodStatus for the extended resource requests backed by DRA. It stores the generated name for the corresponding special ResourceClaim created by the scheduler.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodExtendedResourceClaimStatusAc {
    /// RequestMappings identifies the mapping of \<container, extended resource backed by DRA\> to  device request in the generated ResourceClaim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_mappings: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::ContainerExtendedResourceRequest as crate::Optionable>::Optioned,
        >,
    >,
    /// ResourceClaimName is the name of the ResourceClaim that was generated for the Pod in the namespace of the Pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claim_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::core::v1::PodExtendedResourceClaimStatus {
    type Optioned = PodExtendedResourceClaimStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PodExtendedResourceClaimStatusAc {
    type Optioned = PodExtendedResourceClaimStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::PodExtendedResourceClaimStatus {
    fn into_optioned(self) -> PodExtendedResourceClaimStatusAc {
        PodExtendedResourceClaimStatusAc {
            request_mappings: Some(
                crate::OptionableConvert::into_optioned(self.request_mappings),
            ),
            resource_claim_name: Some(self.resource_claim_name),
        }
    }
    fn try_from_optioned(
        value: PodExtendedResourceClaimStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            request_mappings: crate::OptionableConvert::try_from_optioned(
                value
                    .request_mappings
                    .ok_or(crate::Error {
                        missing_field: "request_mappings",
                    })?,
            )?,
            resource_claim_name: value
                .resource_claim_name
                .ok_or(crate::Error {
                    missing_field: "resource_claim_name",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: PodExtendedResourceClaimStatusAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.request_mappings {
            self.request_mappings = other_value;
        }
        if let Some(other_value) = other.resource_claim_name {
            self.resource_claim_name = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::core::v1::PodExtendedResourceClaimStatus,
> for PodExtendedResourceClaimStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::PodExtendedResourceClaimStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::PodExtendedResourceClaimStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PodExtendedResourceClaimStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
