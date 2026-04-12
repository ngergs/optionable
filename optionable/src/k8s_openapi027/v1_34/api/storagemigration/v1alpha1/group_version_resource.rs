#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// The names of the group, the version, and the resource.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GroupVersionResourceAc {
    /// The name of the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<std::string::String>,
    /// The name of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<std::string::String>,
    /// The name of the version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::storagemigration::v1alpha1::GroupVersionResource {
    type Optioned = GroupVersionResourceAc;
}
#[automatically_derived]
impl crate::Optionable for GroupVersionResourceAc {
    type Optioned = GroupVersionResourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::storagemigration::v1alpha1::GroupVersionResource {
    fn into_optioned(self) -> GroupVersionResourceAc {
        GroupVersionResourceAc {
            group: self.group,
            resource: self.resource,
            version: self.version,
        }
    }
    fn try_from_optioned(value: GroupVersionResourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            group: value.group,
            resource: value.resource,
            version: value.version,
        })
    }
    fn merge(&mut self, other: GroupVersionResourceAc) -> Result<(), crate::Error> {
        if self.group.is_none() {
            self.group = other.group;
        }
        if let Some(other_value) = other.group {
            crate::OptionableConvert::merge(&mut self.group, other_value)?;
        }
        if self.resource.is_none() {
            self.resource = other.resource;
        }
        if let Some(other_value) = other.resource {
            crate::OptionableConvert::merge(&mut self.resource, other_value)?;
        }
        if self.version.is_none() {
            self.version = other.version;
        }
        if let Some(other_value) = other.version {
            crate::OptionableConvert::merge(&mut self.version, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::storagemigration::v1alpha1::GroupVersionResource,
> for GroupVersionResourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::storagemigration::v1alpha1::GroupVersionResource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::storagemigration::v1alpha1::GroupVersionResource,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::storagemigration::v1alpha1::GroupVersionResource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
