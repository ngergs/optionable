#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRequestAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_resources: <Option<
        ::k8s_openapi::api::resource::v1alpha2::NamedResourcesRequest,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_parameters: <Option<
        ::k8s_openapi::apimachinery::pkg::runtime::RawExtension,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha2::ResourceRequest {
    type Optioned = ResourceRequestAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceRequestAc {
    type Optioned = ResourceRequestAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha2::ResourceRequest {
    fn into_optioned(self) -> ResourceRequestAc {
        ResourceRequestAc {
            named_resources: crate::OptionableConvert::into_optioned(
                self.named_resources,
            ),
            vendor_parameters: crate::OptionableConvert::into_optioned(
                self.vendor_parameters,
            ),
        }
    }
    fn try_from_optioned(value: ResourceRequestAc) -> Result<Self, crate::Error> {
        Ok(Self {
            named_resources: crate::OptionableConvert::try_from_optioned(
                value.named_resources,
            )?,
            vendor_parameters: crate::OptionableConvert::try_from_optioned(
                value.vendor_parameters,
            )?,
        })
    }
    fn merge(&mut self, other: ResourceRequestAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.named_resources,
            other.named_resources,
        )?;
        crate::OptionableConvert::merge(
            &mut self.vendor_parameters,
            other.vendor_parameters,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::resource::v1alpha2::ResourceRequest>
for ResourceRequestAc {
    fn from_optionable(
        value: ::k8s_openapi::api::resource::v1alpha2::ResourceRequest,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::resource::v1alpha2::ResourceRequest, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::resource::v1alpha2::ResourceRequest,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
