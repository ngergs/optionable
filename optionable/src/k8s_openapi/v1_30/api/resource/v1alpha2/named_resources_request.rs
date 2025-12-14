#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NamedResourcesRequestAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1alpha2::NamedResourcesRequest {
    type Optioned = NamedResourcesRequestAc;
}
#[automatically_derived]
impl crate::Optionable for NamedResourcesRequestAc {
    type Optioned = NamedResourcesRequestAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha2::NamedResourcesRequest {
    fn into_optioned(self) -> NamedResourcesRequestAc {
        NamedResourcesRequestAc {
            selector: Some(crate::OptionableConvert::into_optioned(self.selector)),
        }
    }
    fn try_from_optioned(value: NamedResourcesRequestAc) -> Result<Self, crate::Error> {
        Ok(Self {
            selector: crate::OptionableConvert::try_from_optioned(
                value
                    .selector
                    .ok_or(crate::Error {
                        missing_field: "selector",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: NamedResourcesRequestAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.selector {
            crate::OptionableConvert::merge(&mut self.selector, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::api::resource::v1alpha2::NamedResourcesRequest,
> for NamedResourcesRequestAc {
    fn from_optionable(
        value: ::k8s_openapi::api::resource::v1alpha2::NamedResourcesRequest,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::resource::v1alpha2::NamedResourcesRequest,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::resource::v1alpha2::NamedResourcesRequest,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
