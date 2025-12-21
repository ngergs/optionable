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
    pub api_group: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::api::resource::v1beta2::ResourceClaimConsumerReference {
    type Optioned = ResourceClaimConsumerReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimConsumerReferenceAc {
    type Optioned = ResourceClaimConsumerReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::resource::v1beta2::ResourceClaimConsumerReference {
    fn into_optioned(self) -> ResourceClaimConsumerReferenceAc {
        ResourceClaimConsumerReferenceAc {
            api_group: crate::OptionableConvert::into_optioned(self.api_group),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            resource: Some(crate::OptionableConvert::into_optioned(self.resource)),
            uid: Some(crate::OptionableConvert::into_optioned(self.uid)),
        }
    }
    fn try_from_optioned(
        value: ResourceClaimConsumerReferenceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            api_group: crate::OptionableConvert::try_from_optioned(value.api_group)?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
            resource: crate::OptionableConvert::try_from_optioned(
                value
                    .resource
                    .ok_or(crate::Error {
                        missing_field: "resource",
                    })?,
            )?,
            uid: crate::OptionableConvert::try_from_optioned(
                value
                    .uid
                    .ok_or(crate::Error {
                        missing_field: "uid",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceClaimConsumerReferenceAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.api_group, other.api_group)?;
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        if let Some(other_value) = other.resource {
            crate::OptionableConvert::merge(&mut self.resource, other_value)?;
        }
        if let Some(other_value) = other.uid {
            crate::OptionableConvert::merge(&mut self.uid, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::api::resource::v1beta2::ResourceClaimConsumerReference,
> for ResourceClaimConsumerReferenceAc {
    fn from_optionable(
        value: k8s_openapi026::api::resource::v1beta2::ResourceClaimConsumerReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::resource::v1beta2::ResourceClaimConsumerReference,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::resource::v1beta2::ResourceClaimConsumerReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
