#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ResourceClaim references one entry in PodSpec.ResourceClaims.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceClaimAc {
    /// Name must match the name of one entry in pod.spec.resourceClaims of the Pod where this field is used. It makes that resource available inside a container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// Request is the name chosen for a request in the referenced claim. If empty, everything from the claim is made available, otherwise only the result of this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ResourceClaim {
    type Optioned = ResourceClaimAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimAc {
    type Optioned = ResourceClaimAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ResourceClaim {
    fn into_optioned(self) -> ResourceClaimAc {
        ResourceClaimAc {
            name: Some(self.name),
            request: self.request,
        }
    }
    fn try_from_optioned(value: ResourceClaimAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            request: value.request,
        })
    }
    fn merge(&mut self, other: ResourceClaimAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        self.request = other.request;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ResourceClaim>
for ResourceClaimAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::ResourceClaim) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ResourceClaim, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ResourceClaim,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
