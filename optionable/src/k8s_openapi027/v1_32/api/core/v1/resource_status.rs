#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ResourceStatus represents the status of a single resource allocated to a Pod.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceStatusAc {
    /// Name of the resource. Must be unique within the pod and in case of non-DRA resource, match one of the resources from the pod spec. For DRA resources, the value must be "claim:\<claim_name\>/\<request\>". When this status is reported about a container, the "claim_name" and "request" must match one of the claims of this container.
    pub name: std::string::String,
    /// List of unique resources health. Each element in the list contains an unique resource ID and its health. At a minimum, for the lifetime of a Pod, resource ID must uniquely identify the resource allocated to the Pod on the Node. If other Pod on the same Node reports the status with the same resource ID, it must be the same resource they share. See ResourceID type definition for a specific format it has in various use cases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::ResourceHealth as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ResourceStatus {
    type Optioned = ResourceStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceStatusAc {
    type Optioned = ResourceStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ResourceStatus {
    fn into_optioned(self) -> ResourceStatusAc {
        ResourceStatusAc {
            name: self.name,
            resources: crate::OptionableConvert::into_optioned(self.resources),
        }
    }
    fn try_from_optioned(value: ResourceStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value.name,
            resources: crate::OptionableConvert::try_from_optioned(value.resources)?,
        })
    }
    fn merge(&mut self, other: ResourceStatusAc) -> Result<(), crate::Error> {
        self.name = other.name;
        if self.resources.is_none() {
            self.resources = crate::OptionableConvert::try_from_optioned(
                other.resources,
            )?;
        } else if let Some(self_value) = self.resources.as_mut()
            && let Some(other_value) = other.resources
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi027::api::core::v1::ResourceStatus {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.name == other.name
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ResourceStatus>
for ResourceStatusAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::ResourceStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ResourceStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ResourceStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for ResourceStatusAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.name, other.name);
        crate::k8s_openapi::merge::merge_map(&mut self.resources, other.resources);
    }
}
impl crate::merge::MapKeysEq for ResourceStatusAc {
    fn keys_eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
