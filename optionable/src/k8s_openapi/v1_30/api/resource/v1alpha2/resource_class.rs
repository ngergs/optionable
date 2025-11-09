#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClassAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters_ref: <Option<
        ::k8s_openapi::api::resource::v1alpha2::ResourceClassParametersReference,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_parameters: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suitable_nodes: <Option<
        ::k8s_openapi::api::core::v1::NodeSelector,
    > as crate::Optionable>::Optioned,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    pub phantom: std::marker::PhantomData<ResourceClassAc>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha2::ResourceClass {
    type Optioned = ResourceClassAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClassAc {
    type Optioned = ResourceClassAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1alpha2::ResourceClass {
    fn into_optioned(self) -> ResourceClassAc {
        ResourceClassAc {
            driver_name: Some(crate::OptionableConvert::into_optioned(self.driver_name)),
            metadata: self.metadata,
            parameters_ref: crate::OptionableConvert::into_optioned(self.parameters_ref),
            structured_parameters: crate::OptionableConvert::into_optioned(
                self.structured_parameters,
            ),
            suitable_nodes: crate::OptionableConvert::into_optioned(self.suitable_nodes),
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(value: ResourceClassAc) -> Result<Self, crate::Error> {
        Ok(Self {
            driver_name: crate::OptionableConvert::try_from_optioned(
                value
                    .driver_name
                    .ok_or(crate::Error {
                        missing_field: "driver_name",
                    })?,
            )?,
            metadata: value.metadata,
            parameters_ref: crate::OptionableConvert::try_from_optioned(
                value.parameters_ref,
            )?,
            structured_parameters: crate::OptionableConvert::try_from_optioned(
                value.structured_parameters,
            )?,
            suitable_nodes: crate::OptionableConvert::try_from_optioned(
                value.suitable_nodes,
            )?,
        })
    }
    fn merge(&mut self, other: ResourceClassAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.driver_name {
            crate::OptionableConvert::merge(&mut self.driver_name, other_value)?;
        }
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.parameters_ref, other.parameters_ref)?;
        crate::OptionableConvert::merge(
            &mut self.structured_parameters,
            other.structured_parameters,
        )?;
        crate::OptionableConvert::merge(&mut self.suitable_nodes, other.suitable_nodes)?;
        Ok(())
    }
}
impl k8s_openapi::Resource for ResourceClassAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::resource::v1alpha2::ResourceClass as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::resource::v1alpha2::ResourceClass as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::resource::v1alpha2::ResourceClass as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::resource::v1alpha2::ResourceClass as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::resource::v1alpha2::ResourceClass as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::resource::v1alpha2::ResourceClass as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for ResourceClassAc {
    type Ty = <::k8s_openapi::api::resource::v1alpha2::ResourceClass as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
