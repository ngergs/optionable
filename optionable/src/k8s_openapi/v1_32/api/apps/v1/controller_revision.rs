#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ControllerRevisionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: <Option<
        ::k8s_openapi::apimachinery::pkg::runtime::RawExtension,
    > as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    pub phantom: std::marker::PhantomData<ControllerRevisionAc>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::apps::v1::ControllerRevision {
    type Optioned = ControllerRevisionAc;
}
#[automatically_derived]
impl crate::Optionable for ControllerRevisionAc {
    type Optioned = ControllerRevisionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::apps::v1::ControllerRevision {
    fn into_optioned(self) -> ControllerRevisionAc {
        ControllerRevisionAc {
            data: crate::OptionableConvert::into_optioned(self.data),
            metadata: self.metadata,
            revision: Some(self.revision),
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(value: ControllerRevisionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            data: crate::OptionableConvert::try_from_optioned(value.data)?,
            metadata: value.metadata,
            revision: value
                .revision
                .ok_or(crate::Error {
                    missing_field: "revision",
                })?,
        })
    }
    fn merge(&mut self, other: ControllerRevisionAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.data, other.data)?;
        self.metadata = other.metadata;
        if let Some(other_value) = other.revision {
            self.revision = other_value;
        }
        Ok(())
    }
}
impl k8s_openapi::Resource for ControllerRevisionAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::apps::v1::ControllerRevision as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::apps::v1::ControllerRevision as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::apps::v1::ControllerRevision as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::apps::v1::ControllerRevision as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::apps::v1::ControllerRevision as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::apps::v1::ControllerRevision as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for ControllerRevisionAc {
    type Ty = <::k8s_openapi::api::apps::v1::ControllerRevision as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
