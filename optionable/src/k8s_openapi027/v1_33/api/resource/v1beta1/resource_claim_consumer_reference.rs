#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceClaimConsumerReferenceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_group: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::resource::v1beta1::ResourceClaimConsumerReference {
    type Optioned = ResourceClaimConsumerReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimConsumerReferenceAc {
    type Optioned = ResourceClaimConsumerReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1beta1::ResourceClaimConsumerReference {
    fn into_optioned(self) -> ResourceClaimConsumerReferenceAc {
        ResourceClaimConsumerReferenceAc {
            api_group: self.api_group,
            name: Some(self.name),
            resource: Some(self.resource),
            uid: Some(self.uid),
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
            uid: value
                .uid
                .ok_or(crate::Error {
                    missing_field: "uid",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceClaimConsumerReferenceAc,
    ) -> Result<(), crate::Error> {
        self.api_group = other.api_group;
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        if let Some(other_value) = other.resource {
            self.resource = other_value;
        }
        if let Some(other_value) = other.uid {
            self.uid = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::resource::v1beta1::ResourceClaimConsumerReference,
> for ResourceClaimConsumerReferenceAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta1::ResourceClaimConsumerReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1beta1::ResourceClaimConsumerReference,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta1::ResourceClaimConsumerReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
