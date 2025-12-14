#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceClaimParametersReferenceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_group: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1alpha2::ResourceClaimParametersReference {
    type Optioned = ResourceClaimParametersReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimParametersReferenceAc {
    type Optioned = ResourceClaimParametersReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha2::ResourceClaimParametersReference {
    fn into_optioned(self) -> ResourceClaimParametersReferenceAc {
        ResourceClaimParametersReferenceAc {
            api_group: crate::OptionableConvert::into_optioned(self.api_group),
            kind: Some(crate::OptionableConvert::into_optioned(self.kind)),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(
        value: ResourceClaimParametersReferenceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            api_group: crate::OptionableConvert::try_from_optioned(value.api_group)?,
            kind: crate::OptionableConvert::try_from_optioned(
                value
                    .kind
                    .ok_or(crate::Error {
                        missing_field: "kind",
                    })?,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceClaimParametersReferenceAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.api_group, other.api_group)?;
        if let Some(other_value) = other.kind {
            crate::OptionableConvert::merge(&mut self.kind, other_value)?;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::api::resource::v1alpha2::ResourceClaimParametersReference,
> for ResourceClaimParametersReferenceAc {
    fn from_optionable(
        value: ::k8s_openapi::api::resource::v1alpha2::ResourceClaimParametersReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::resource::v1alpha2::ResourceClaimParametersReference,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::resource::v1alpha2::ResourceClaimParametersReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
