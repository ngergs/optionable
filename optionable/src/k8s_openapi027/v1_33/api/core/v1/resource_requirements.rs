#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ResourceRequirements describes the compute resource requirements.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceRequirementsAc {
    /// Claims lists the names of resources, defined in spec.resourceClaims, that are used by this container.
    ///
    /// This is an alpha field and requires enabling the DynamicResourceAllocation feature gate.
    ///
    /// This field is immutable. It can only be set for containers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::ResourceClaim as crate::Optionable>::Optioned,
        >,
    >,
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. Requests cannot exceed Limits. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ResourceRequirements {
    type Optioned = ResourceRequirementsAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceRequirementsAc {
    type Optioned = ResourceRequirementsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ResourceRequirements {
    fn into_optioned(self) -> ResourceRequirementsAc {
        ResourceRequirementsAc {
            claims: crate::OptionableConvert::into_optioned(self.claims),
            limits: crate::OptionableConvert::into_optioned(self.limits),
            requests: crate::OptionableConvert::into_optioned(self.requests),
        }
    }
    fn try_from_optioned(value: ResourceRequirementsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            claims: crate::OptionableConvert::try_from_optioned(value.claims)?,
            limits: crate::OptionableConvert::try_from_optioned(value.limits)?,
            requests: crate::OptionableConvert::try_from_optioned(value.requests)?,
        })
    }
    fn merge(&mut self, other: ResourceRequirementsAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.claims, other.claims)?;
        crate::OptionableConvert::merge(&mut self.limits, other.limits)?;
        crate::OptionableConvert::merge(&mut self.requests, other.requests)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ResourceRequirements>
for ResourceRequirementsAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ResourceRequirements,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ResourceRequirements, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ResourceRequirements,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
