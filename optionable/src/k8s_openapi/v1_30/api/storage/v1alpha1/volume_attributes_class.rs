#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttributesClassAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        deserialize_with = "crate::k8s_openapi::deserialize_api_envelope"
    )]
    pub phantom: std::marker::PhantomData<Self>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1alpha1::VolumeAttributesClass {
    type Optioned = VolumeAttributesClassAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeAttributesClassAc {
    type Optioned = VolumeAttributesClassAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::storage::v1alpha1::VolumeAttributesClass {
    fn into_optioned(self) -> VolumeAttributesClassAc {
        VolumeAttributesClassAc {
            driver_name: Some(crate::OptionableConvert::into_optioned(self.driver_name)),
            metadata: self.metadata,
            parameters: crate::OptionableConvert::into_optioned(self.parameters),
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(value: VolumeAttributesClassAc) -> Result<Self, crate::Error> {
        Ok(Self {
            driver_name: crate::OptionableConvert::try_from_optioned(
                value
                    .driver_name
                    .ok_or(crate::Error {
                        missing_field: "driver_name",
                    })?,
            )?,
            metadata: value.metadata,
            parameters: crate::OptionableConvert::try_from_optioned(value.parameters)?,
        })
    }
    fn merge(&mut self, other: VolumeAttributesClassAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.driver_name {
            crate::OptionableConvert::merge(&mut self.driver_name, other_value)?;
        }
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.parameters, other.parameters)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::storage::v1alpha1::VolumeAttributesClass>
for VolumeAttributesClassAc {
    fn from_optionable(
        value: ::k8s_openapi::api::storage::v1alpha1::VolumeAttributesClass,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::storage::v1alpha1::VolumeAttributesClass,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::storage::v1alpha1::VolumeAttributesClass,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi::Resource for VolumeAttributesClassAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::storage::v1alpha1::VolumeAttributesClass as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::storage::v1alpha1::VolumeAttributesClass as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::storage::v1alpha1::VolumeAttributesClass as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::storage::v1alpha1::VolumeAttributesClass as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::storage::v1alpha1::VolumeAttributesClass as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::storage::v1alpha1::VolumeAttributesClass as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for VolumeAttributesClassAc {
    type Ty = <::k8s_openapi::api::storage::v1alpha1::VolumeAttributesClass as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_volumeattributesclassac() {
    crate::testutil::roundtrip_test::<
        ::k8s_openapi::api::storage::v1alpha1::VolumeAttributesClass,
    >();
}
