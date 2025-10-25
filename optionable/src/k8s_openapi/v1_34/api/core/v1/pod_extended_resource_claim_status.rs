pub struct PodExtendedResourceClaimStatusOpt {
    pub request_mappings: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::core::v1::ContainerExtendedResourceRequest,
        > as crate::Optionable>::Optioned,
    >,
    pub resource_claim_name: Option<
        <std::string::String as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::pod_extended_resource_claim_status::PodExtendedResourceClaimStatus {
    type Optioned = PodExtendedResourceClaimStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for PodExtendedResourceClaimStatusOpt {
    type Optioned = PodExtendedResourceClaimStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::pod_extended_resource_claim_status::PodExtendedResourceClaimStatus {
    fn into_optioned(self) -> PodExtendedResourceClaimStatusOpt {
        PodExtendedResourceClaimStatusOpt {
            request_mappings: Some(
                <std::vec::Vec<
                    ::k8s_openapi::api::core::v1::ContainerExtendedResourceRequest,
                > as crate::OptionableConvert>::into_optioned(self.request_mappings),
            ),
            resource_claim_name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.resource_claim_name,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: PodExtendedResourceClaimStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            request_mappings: <std::vec::Vec<
                ::k8s_openapi::api::core::v1::ContainerExtendedResourceRequest,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .request_mappings
                    .ok_or(crate::optionable::Error {
                        missing_field: "request_mappings",
                    })?,
            )?,
            resource_claim_name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .resource_claim_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "resource_claim_name",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PodExtendedResourceClaimStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.request_mappings {
            <std::vec::Vec<
                ::k8s_openapi::api::core::v1::ContainerExtendedResourceRequest,
            > as crate::OptionableConvert>::merge(
                &mut self.request_mappings,
                other_value,
            )?;
        }
        if let Some(other_value) = other.resource_claim_name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.resource_claim_name,
                other_value,
            )?;
        }
        Ok(())
    }
}
