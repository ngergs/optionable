#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClassParametersAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1alpha2::ResourceFilter>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_from: <Option<
        ::k8s_openapi::api::resource::v1alpha2::ResourceClassParametersReference,
    > as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_parameters: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1alpha2::VendorParameters>,
    > as crate::Optionable>::Optioned,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    pub phantom: std::marker::PhantomData<ResourceClassParametersAc>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1alpha2::ResourceClassParameters {
    type Optioned = ResourceClassParametersAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClassParametersAc {
    type Optioned = ResourceClassParametersAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha2::ResourceClassParameters {
    fn into_optioned(self) -> ResourceClassParametersAc {
        ResourceClassParametersAc {
            filters: crate::OptionableConvert::into_optioned(self.filters),
            generated_from: crate::OptionableConvert::into_optioned(self.generated_from),
            metadata: self.metadata,
            vendor_parameters: crate::OptionableConvert::into_optioned(
                self.vendor_parameters,
            ),
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(
        value: ResourceClassParametersAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            filters: crate::OptionableConvert::try_from_optioned(value.filters)?,
            generated_from: crate::OptionableConvert::try_from_optioned(
                value.generated_from,
            )?,
            metadata: value.metadata,
            vendor_parameters: crate::OptionableConvert::try_from_optioned(
                value.vendor_parameters,
            )?,
        })
    }
    fn merge(&mut self, other: ResourceClassParametersAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.filters, other.filters)?;
        crate::OptionableConvert::merge(&mut self.generated_from, other.generated_from)?;
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(
            &mut self.vendor_parameters,
            other.vendor_parameters,
        )?;
        Ok(())
    }
}
impl k8s_openapi::Resource for ResourceClassParametersAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::resource::v1alpha2::ResourceClassParameters as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::resource::v1alpha2::ResourceClassParameters as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::resource::v1alpha2::ResourceClassParameters as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::resource::v1alpha2::ResourceClassParameters as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::resource::v1alpha2::ResourceClassParameters as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::resource::v1alpha2::ResourceClassParameters as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for ResourceClassParametersAc {
    type Ty = <::k8s_openapi::api::resource::v1alpha2::ResourceClassParameters as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
