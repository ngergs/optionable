#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ResourceClaimConsumerReference contains enough information to let you locate the consumer of a ResourceClaim. The user must be a resource in the same namespace as the ResourceClaim.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceClaimConsumerReferenceAc {
    /// APIGroup is the group for the resource being referenced. It is empty for the core API. This matches the group in the APIVersion that is used when creating the resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_group: Option<std::string::String>,
    /// Name is the name of resource being referenced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// Resource is the type of resource being referenced, for example "pods".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<std::string::String>,
    /// UID identifies exactly one incarnation of the resource.
    pub uid: std::string::String,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::resource::v1beta2::ResourceClaimConsumerReference {
    type Optioned = ResourceClaimConsumerReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimConsumerReferenceAc {
    type Optioned = ResourceClaimConsumerReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1beta2::ResourceClaimConsumerReference {
    fn into_optioned(self) -> ResourceClaimConsumerReferenceAc {
        ResourceClaimConsumerReferenceAc {
            api_group: self.api_group,
            name: Some(self.name),
            resource: Some(self.resource),
            uid: self.uid,
        }
    }
    fn try_from_optioned(
        value: ResourceClaimConsumerReferenceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            api_group: value.api_group,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            resource: value
                .resource
                .ok_or(crate::Error {
                    missing_field: "resource",
                })?,
            uid: value.uid,
        })
    }
    fn merge(
        &mut self,
        other: ResourceClaimConsumerReferenceAc,
    ) -> Result<(), crate::Error> {
        if self.api_group.is_none() {
            self.api_group = crate::OptionableConvert::try_from_optioned(
                other.api_group,
            )?;
        } else if let Some(self_value) = self.api_group.as_mut()
            && let Some(other_value) = other.api_group
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.name {
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.resource {
            self.resource = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        self.uid = other.uid;
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi027::api::resource::v1beta2::ResourceClaimConsumerReference {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.uid == other.uid
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::resource::v1beta2::ResourceClaimConsumerReference,
> for ResourceClaimConsumerReferenceAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta2::ResourceClaimConsumerReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1beta2::ResourceClaimConsumerReference,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta2::ResourceClaimConsumerReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for ResourceClaimConsumerReferenceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.api_group, other.api_group);
        k8s_openapi027::DeepMerge::merge_from(&mut self.name, other.name);
        k8s_openapi027::DeepMerge::merge_from(&mut self.resource, other.resource);
        k8s_openapi027::DeepMerge::merge_from(&mut self.uid, other.uid);
    }
}
impl crate::merge::MapKeysEq for ResourceClaimConsumerReferenceAc {
    fn keys_eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}
