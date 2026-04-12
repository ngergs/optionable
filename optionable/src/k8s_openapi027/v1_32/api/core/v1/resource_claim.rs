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
    pub name: std::string::String,
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
            name: self.name,
            request: self.request,
        }
    }
    fn try_from_optioned(value: ResourceClaimAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value.name,
            request: value.request,
        })
    }
    fn merge(&mut self, other: ResourceClaimAc) -> Result<(), crate::Error> {
        self.name = other.name;
        if self.request.is_none() {
            self.request = other.request;
        }
        if let Some(other_value) = other.request {
            crate::OptionableConvert::merge(&mut self.request, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq for k8s_openapi027::api::core::v1::ResourceClaim {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.name == other.name
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
