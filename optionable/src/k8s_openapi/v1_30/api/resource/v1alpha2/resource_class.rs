#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceClassAc {
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_api_version",
        deserialize_with = "crate::k8s_openapi::deserialize_api_version"
    )]
    pub api_version: std::marker::PhantomData<Self>,
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_kind",
        deserialize_with = "crate::k8s_openapi::deserialize_kind"
    )]
    pub kind: std::marker::PhantomData<Self>,
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
            api_version: Default::default(),
            kind: Default::default(),
            driver_name: Some(crate::OptionableConvert::into_optioned(self.driver_name)),
            metadata: self.metadata,
            parameters_ref: crate::OptionableConvert::into_optioned(self.parameters_ref),
            structured_parameters: crate::OptionableConvert::into_optioned(
                self.structured_parameters,
            ),
            suitable_nodes: crate::OptionableConvert::into_optioned(self.suitable_nodes),
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::resource::v1alpha2::ResourceClass>
for ResourceClassAc {
    fn from_optionable(
        value: ::k8s_openapi::api::resource::v1alpha2::ResourceClass,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::resource::v1alpha2::ResourceClass, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::resource::v1alpha2::ResourceClass,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
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
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_resourceclassac() {
    crate::testutil::roundtrip_test::<
        ::k8s_openapi::api::resource::v1alpha2::ResourceClass,
    >();
}
