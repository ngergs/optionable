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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// List of unique resources health. Each element in the list contains an unique resource ID and its health. At a minimum, for the lifetime of a Pod, resource ID must uniquely identify the resource allocated to the Pod on the Node. If other Pod on the same Node reports the status with the same resource ID, it must be the same resource they share. See ResourceID type definition for a specific format it has in various use cases.
    pub resources: Option<
        std::vec::Vec<::k8s_openapi027::api::core::v1::ResourceHealth>,
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
            name: Some(self.name),
            resources: self.resources,
        }
    }
    fn try_from_optioned(value: ResourceStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            resources: value.resources,
        })
    }
    fn merge(&mut self, other: ResourceStatusAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        self.resources = other.resources;
        Ok(())
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
