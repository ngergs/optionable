#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
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
        skip_deserializing
    )]
    phantom: std::marker::PhantomData<VolumeAttributesClassAc>,
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
impl crate::OptionableConvert
for ::k8s_openapi::api::storage::v1alpha1::VolumeAttributesClass {
    fn into_optioned(self) -> VolumeAttributesClassAc {
        VolumeAttributesClassAc {
            driver_name: Some(crate::OptionableConvert::into_optioned(self.driver_name)),
            metadata: self.metadata,
            parameters: crate::OptionableConvert::into_optioned(self.parameters),
        }
    }
    fn try_from_optioned(
        value: VolumeAttributesClassAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            driver_name: crate::OptionableConvert::try_from_optioned(
                value
                    .driver_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "driver_name",
                    })?,
            )?,
            metadata: value.metadata,
            parameters: crate::OptionableConvert::try_from_optioned(value.parameters)?,
        })
    }
    fn merge(
        &mut self,
        other: VolumeAttributesClassAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.driver_name {
            crate::OptionableConvert::merge(&mut self.driver_name, other_value)?;
        }
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.parameters, other.parameters)?;
        Ok(())
    }
}
impl k8s_openapi::Resource for VolumeAttributesClassAc {
    const API_VERSION: &'static str = "storage.k8s.io/v1alpha1";
    const GROUP: &'static str = "storage.k8s.io";
    const KIND: &'static str = "VolumeAttributesClass";
    const VERSION: &'static str = "v1alpha1";
    const URL_PATH_SEGMENT: &'static str = "volumeattributesclasses";
    type Scope = k8s_openapi::ClusterResourceScope;
}
impl k8s_openapi::Metadata for VolumeAttributesClassAc {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
